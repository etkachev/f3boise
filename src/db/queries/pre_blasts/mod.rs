use crate::shared::common_errors::AppError;
use chrono::{NaiveDate, NaiveTime};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

/// full json representation of preblast data in db
pub struct PreBlastJsonFullData {
    pub id: Uuid,
    pub ao: String,
    pub channel_id: String,
    pub title: String,
    pub qs: Vec<String>,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub why: String,
    pub equipment: Option<Vec<String>>,
    pub fng_message: Option<String>,
    pub mole_skin: Option<String>,
    pub img_ids: Option<Vec<String>>,
    pub ts: Option<String>,
}

/// get all pre_blasts from db. mainly for syncing database
pub async fn get_all_pre_blasts(db: &PgPool) -> Result<Vec<PreBlastJsonFullData>, AppError> {
    let results: Vec<PreBlastJsonFullData> = sqlx::query_as!(
        PreBlastJsonFullData,
        r#"
    WITH list_view AS (
        SELECT
            pb.id as id,
            al.name as ao,
            pb.channel_id,
            pb.title,
            string_to_array(lower(qs), ',') as qs,
            date,
            pb.start_time,
            pb.why,
            string_to_array(COALESCE(pb.equipment, ''), ',') as equipment,
            pb.fng_message,
            pb.mole_skin,
            string_to_array(COALESCE(pb.img_ids, ''), ',') as img_ids,
            pb.ts
        FROM pre_blasts pb
        INNER JOIN ao_list al on pb.channel_id = al.channel_id
    )

    SELECT
        id,
        ao,
        channel_id,
        title,
        qs as "qs!",
        date,
        start_time,
        why,
        equipment as "equipment!",
        fng_message,
        mole_skin,
        img_ids as "img_ids!",
        ts
    FROM list_view
    ORDER BY date DESC;
    "#
    )
    .fetch_all(db)
    .await?;

    Ok(results)
}

/// get preblast data by id
pub async fn get_pre_blast_by_id(
    db: &PgPool,
    id: &str,
) -> Result<Option<PreBlastJsonFullData>, AppError> {
    let id = Uuid::from_str(id)?;
    let result: Option<PreBlastJsonFullData> = sqlx::query_as!(
        PreBlastJsonFullData,
        r#"
    WITH list_view AS (
        SELECT
            pb.id as id,
            al.name as ao,
            pb.channel_id,
            pb.title,
            string_to_array(lower(qs), ',') as qs,
            date,
            pb.start_time,
            pb.why,
            string_to_array(COALESCE(pb.equipment, ''), ',') as equipment,
            pb.fng_message,
            pb.mole_skin,
            string_to_array(COALESCE(pb.img_ids, ''), ',') as img_ids,
            pb.ts
        FROM pre_blasts pb
        INNER JOIN ao_list al on pb.channel_id = al.channel_id
    )

    SELECT
        id,
        ao,
        channel_id,
        title,
        qs as "qs!",
        date,
        start_time,
        why,
        equipment as "equipment!",
        fng_message,
        mole_skin,
        img_ids as "img_ids!",
        ts
    FROM list_view
    WHERE id = $1
    ORDER BY date DESC;
    "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(result)
}
