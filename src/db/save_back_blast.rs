use crate::app_state::backblast_data::BackBlastData;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use uuid::Uuid;

/// db representation of back blast
pub struct BackBlastDbEntry {
    pub id: Uuid,
    pub ao: String,
    pub q: String,
    pub pax: String,
    pub date: NaiveDate,
    pub bb_type: String,
    pub channel_id: String,
    pub active: bool,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    pub fngs: Option<String>,
}

impl From<&BackBlastData> for BackBlastDbEntry {
    fn from(data: &BackBlastData) -> Self {
        let mut q: Vec<String> = data.qs.clone().into_iter().collect();
        let mut pax: Vec<String> = data.get_pax().into_iter().collect();
        let mut fngs: Vec<String> = data.fngs.clone().into_iter().collect();
        q.sort();
        pax.sort();
        fngs.sort();
        BackBlastDbEntry {
            id: Uuid::new_v4(),
            ao: data.ao.to_string(),
            date: data.date,
            q: q.join(","),
            pax: pax.join(","),
            bb_type: data.bb_type.to_string(),
            channel_id: data.ao.channel_id().to_string(),
            active: !data.ao.is_otb(),
            title: data.title.clone(),
            moleskine: data.moleskine.clone(),
            fngs: Some(fngs.join(",")),
        }
    }
}

/// save backblasts to db.
pub async fn save_multiple(db_pool: &PgPool, list: &[BackBlastData]) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for back_blast in list {
        let db_bb = BackBlastDbEntry::from(back_blast);
        save_back_blast(&mut transaction, &db_bb).await?;
    }

    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

/// sync backblasts to db (for syncing from other DB purposes).
pub async fn sync_multiple(db_pool: &PgPool, list: &[BackBlastData]) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    for back_blast in list {
        let db_bb = BackBlastDbEntry::from(back_blast);
        sync_back_blast(&mut transaction, &db_bb).await?;
    }

    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

pub async fn save_single(db_pool: &PgPool, data: &BackBlastData) -> Result<String, AppError> {
    let db_bb = BackBlastDbEntry::from(data);
    let id = db_bb.id.to_string();
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    save_back_blast(&mut transaction, &db_bb).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(id)
}

/// update timestamp for backblast, to be able to edit most recent message post.
pub async fn update_back_blast_ts(db_pool: &PgPool, id: &str, ts: String) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    let uuid = Uuid::from_str(id)?;
    sqlx::query!(
        r#"
    UPDATE back_blasts
    SET ts = $2
    WHERE id = $1
    "#,
        uuid,
        ts
    )
    .execute(&mut *transaction)
    .await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

/// update single back blast by id
pub async fn update_back_blast(
    db_pool: &PgPool,
    id: &str,
    data: &BackBlastData,
) -> Result<(), AppError> {
    let uuid = Uuid::from_str(id)?;
    let db_entry = BackBlastDbEntry::from(data);
    sqlx::query!(
        r#"
    UPDATE back_blasts
    SET ao = $2,
        q = $3,
        pax = $4,
        date = $5,
        channel_id = $6,
        active = $7,
        title = $8,
        moleskine = $9,
        fngs = $10
    WHERE id = $1
    "#,
        uuid,
        db_entry.ao,
        db_entry.q,
        db_entry.pax,
        db_entry.date,
        db_entry.channel_id,
        db_entry.active,
        db_entry.title,
        db_entry.moleskine,
        db_entry.fngs
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

/// insert backblast if not constraint on ao, date, and bb_type
async fn save_back_blast(
    transaction: &mut Transaction<'_, Postgres>,
    db_bb: &BackBlastDbEntry,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    INSERT INTO back_blasts (id, ao, q, pax, date, bb_type, channel_id, active, title, moleskine, fngs)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
    ON CONFLICT ON CONSTRAINT back_blasts_channel_id_date_bb_type_key
        DO NOTHING;
    "#,
        db_bb.id,
        db_bb.ao,
        db_bb.q,
        db_bb.pax,
        db_bb.date,
        db_bb.bb_type,
        db_bb.channel_id,
        db_bb.active,
        db_bb.title,
        db_bb.moleskine,
        db_bb.fngs
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

/// update backblast with expectation on sync from other db
async fn sync_back_blast(
    transaction: &mut Transaction<'_, Postgres>,
    db_bb: &BackBlastDbEntry,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    INSERT INTO back_blasts (id, ao, q, pax, date, bb_type, channel_id, active, title, moleskine, fngs)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
    ON CONFLICT ON CONSTRAINT back_blasts_channel_id_date_bb_type_key
        DO UPDATE
    SET ao = EXCLUDED.ao,
        q = EXCLUDED.q,
        pax = EXCLUDED.pax,
        date = EXCLUDED.date,
        bb_type = EXCLUDED.bb_type,
        channel_id = EXCLUDED.channel_id,
        active = EXCLUDED.active,
        title = EXCLUDED.title,
        moleskine = EXCLUDED.moleskine,
        fngs = EXCLUDED.fngs;
    "#,
        db_bb.id,
        db_bb.ao,
        db_bb.q,
        db_bb.pax,
        db_bb.date,
        db_bb.bb_type,
        db_bb.channel_id,
        db_bb.active,
        db_bb.title,
        db_bb.moleskine,
        db_bb.fngs
    )
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
