use crate::app_state::backblast_data::BackBlastType;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub mod back_blasts_by_ao;
pub mod calculate_bb_list_stats;
pub mod recent_bd_for_pax;

/// get all back blast data (with type 'backblast')
pub async fn get_all(db_pool: &PgPool) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_all_by_type(db_pool, BackBlastType::BackBlast).await
}

/// get all double downs
pub async fn get_all_dd(db_pool: &PgPool) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_all_by_type(db_pool, BackBlastType::DoubleDown).await
}

/// get all backblast db data (double downs included) for purposes of csv download backup.
pub async fn get_full_db_back_blasts(
    db_pool: &PgPool,
) -> Result<Vec<BackBlastFullJsonData>, AppError> {
    let rows: Vec<BackBlastFullJsonData> = sqlx::query_as!(
        BackBlastFullJsonData,
        r#"
        WITH list_view AS (
        SELECT
            bb.id as id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id,
            bb.title,
            bb.moleskine,
            string_to_array(lower(fngs), ',') as fngs,
            bb.ts
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type, title, moleskine, fngs, ts
    FROM list_view
    ORDER BY date DESC;
        "#,
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// get back blast by id
pub async fn get_back_blast_by_id(
    db_pool: &PgPool,
    id: &str,
) -> Result<Option<BackBlastFullJsonData>, AppError> {
    let id = Uuid::from_str(id)?;
    let result: Option<BackBlastFullJsonData> = sqlx::query_as!(
        BackBlastFullJsonData,
        r#"
    WITH list_view AS (
        SELECT
            bb.id as id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id,
            bb.title,
            bb.moleskine,
            string_to_array(lower(fngs), ',') as fngs,
            bb.ts
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type, title, moleskine, fngs, ts
    FROM list_view
    WHERE id = $1
    ORDER BY date DESC;
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(result)
}

async fn get_all_by_type(
    db_pool: &PgPool,
    bb_type: BackBlastType,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    let bb_type = bb_type.to_string();
    let rows: Vec<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            bb.id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = $1 AND bb.active = true
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#,
        bb_type
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// get all back_blasts within a date range
pub async fn get_all_within_date_range(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_all_with_date_range_by_type(db_pool, start_date, end_date, BackBlastType::BackBlast).await
}

/// get all double downs within a date range
pub async fn get_all_dd_within_date_range(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_all_with_date_range_by_type(db_pool, start_date, end_date, BackBlastType::DoubleDown).await
}

async fn get_all_with_date_range_by_type(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    bb_type: BackBlastType,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    let bb_type = bb_type.to_string();
    let rows: Vec<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            bb.id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = $1 AND bb.active = true AND bb.date >= $2 AND bb.date <= $3
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#,
        bb_type,
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// representation of backblast json in db
#[derive(Deserialize, Serialize, Debug)]
pub struct BackBlastJsonData {
    pub id: Uuid,
    pub ao: String,
    pub channel_id: String,
    pub q: Vec<String>,
    pub pax: Vec<String>,
    pub date: NaiveDate,
    pub bb_type: String,
}

/// full representation for back blast in db
#[derive(Deserialize, Serialize, Debug)]
pub struct BackBlastFullJsonData {
    pub id: Uuid,
    pub ao: String,
    pub channel_id: String,
    pub q: Vec<String>,
    pub pax: Vec<String>,
    pub date: NaiveDate,
    pub bb_type: String,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    pub fngs: Option<Vec<String>>,
    pub ts: Option<String>,
}

/// get list of BD's where pax (name passed in) is included.
pub async fn get_list_with_pax(
    db_pool: &PgPool,
    name: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_list_with_pax_by_type(db_pool, name, BackBlastType::BackBlast).await
}

/// get list of DoubleDowns's where pax (name passed in) is included.
pub async fn get_dd_list_with_pax(
    db_pool: &PgPool,
    name: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    get_list_with_pax_by_type(db_pool, name, BackBlastType::DoubleDown).await
}

/// get list of BD's where pax (name passed in) is included.
async fn get_list_with_pax_by_type(
    db_pool: &PgPool,
    name: &str,
    bb_type: BackBlastType,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    let name = name.to_lowercase();
    let bb_type = bb_type.to_string();
    let rows: Vec<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            bb.id,
            al.name as ao,
            string_to_array(lower(q), ',') as q,
            string_to_array(lower(pax), ',') as pax,
            date,
            bb_type,
            bb.channel_id
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = $1 AND bb.active = true
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    WHERE pax @> array[$2]
    ORDER BY date DESC;
    "#,
        bb_type,
        name
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}
