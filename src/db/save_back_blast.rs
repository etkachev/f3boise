use crate::app_state::backblast_data::BackBlastData;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, Transaction};
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
}

impl From<&BackBlastData> for BackBlastDbEntry {
    fn from(data: &BackBlastData) -> Self {
        let mut q: Vec<String> = data.qs.clone().into_iter().collect();
        let mut pax: Vec<String> = data.get_pax().into_iter().collect();
        q.sort();
        pax.sort();
        BackBlastDbEntry {
            id: Uuid::new_v4(),
            ao: data.ao.to_string(),
            date: data.date,
            q: q.join(","),
            pax: pax.join(","),
            // TODO defaults to backblast type.
            bb_type: data.bb_type.to_string(),
            channel_id: data.ao.channel_id().to_string(),
            active: !data.ao.is_otb(),
        }
    }
}

/// save backblasts to db.
pub async fn save(db_pool: &PgPool, list: &[BackBlastData]) -> Result<(), AppError> {
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

/// insert backblast if not constraint on ao, date, and bb_type
async fn save_back_blast(
    transaction: &mut Transaction<'_, Postgres>,
    db_bb: &BackBlastDbEntry,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    INSERT INTO back_blasts (id, ao, q, pax, date, bb_type, channel_id, active)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8)
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
        db_bb.active
    )
    .execute(transaction)
    .await?;

    Ok(())
}
