use crate::app_state::pre_blast_data::PreBlastData;
use crate::shared::common_errors::AppError;
use chrono::{NaiveDate, NaiveTime};
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use uuid::Uuid;

/// db representation of back blast for inserting
pub struct PreBlastDbEntry {
    pub id: Uuid,
    pub ao: String,
    pub channel_id: String,
    pub title: String,
    pub qs: String,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub why: String,
    pub equipment: Option<String>,
    pub fng_message: Option<String>,
    pub mole_skin: Option<String>,
    pub img_ids: Option<String>,
    pub ts: Option<String>,
}

impl From<&PreBlastData> for PreBlastDbEntry {
    fn from(value: &PreBlastData) -> Self {
        let mut qs: Vec<String> = value.qs.clone().into_iter().collect();
        qs.sort();

        let equipment = if value.equipment.is_empty() {
            None
        } else {
            Some(
                value
                    .equipment
                    .iter()
                    .map(|eq| eq.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
        };

        let img_ids = if value.img_ids.is_empty() {
            None
        } else {
            Some(
                value
                    .img_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
        };

        PreBlastDbEntry {
            id: Uuid::new_v4(),
            ao: value.ao.to_string(),
            channel_id: value.ao.channel_id().to_string(),
            title: value.title.to_string(),
            qs: qs.join(","),
            date: value.date,
            start_time: value.start_time,
            why: value.why.to_string(),
            equipment,
            fng_message: value.fng_message.clone(),
            mole_skin: value.mole_skin.clone(),
            img_ids,
            ts: None,
        }
    }
}

pub async fn save_single(db_pool: &PgPool, data: &PreBlastData) -> Result<String, AppError> {
    let db_pb = PreBlastDbEntry::from(data);
    let id = db_pb.id.to_string();

    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");

    save_pre_blast(&mut transaction, &db_pb).await?;

    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(id)
}

/// update timestamp for pre-blast, to be able to edit most recent message post.
pub async fn update_pre_blast_ts(db_pool: &PgPool, id: &str, ts: String) -> Result<(), AppError> {
    let mut transaction = db_pool.begin().await.expect("Failed to begin transaction");
    let uuid = Uuid::from_str(id)?;

    sqlx::query!(
        r#"
    UPDATE pre_blasts
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

/// update full pre blast data
pub async fn update_pre_blast(
    db_pool: &PgPool,
    id: &str,
    pre_blast: &PreBlastData,
) -> Result<(), AppError> {
    let uuid = Uuid::from_str(id)?;
    let db_entry = PreBlastDbEntry::from(pre_blast);
    sqlx::query!(
        r#"
     UPDATE pre_blasts
     SET ao = $2,
         channel_id = $3,
         title = $4,
         qs = $5,
         date = $6,
         start_time = $7,
         why = $8,
         equipment = $9,
         fng_message = $10,
         mole_skin = $11,
         img_ids = $12
     WHERE id = $1
     "#,
        uuid,
        db_entry.ao,
        db_entry.channel_id,
        db_entry.title,
        db_entry.qs,
        db_entry.date,
        db_entry.start_time,
        db_entry.why,
        db_entry.equipment,
        db_entry.fng_message,
        db_entry.mole_skin,
        db_entry.img_ids,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

async fn save_pre_blast(
    transaction: &mut Transaction<'_, Postgres>,
    entry: &PreBlastDbEntry,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO pre_blasts (id, ao, channel_id, title, qs, date, start_time, why, equipment, fng_message, mole_skin, img_ids)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        entry.id,
        entry.ao,
        entry.channel_id,
        entry.title,
        entry.qs,
        entry.date,
        entry.start_time,
        entry.why,
        entry.equipment,
        entry.fng_message,
        entry.mole_skin,
        entry.img_ids
    )
        .execute(&mut **transaction)
        .await?;
    Ok(())
}
