use crate::app_state::backblast_data::BackBlastType;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
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
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
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

#[derive(Deserialize, Serialize, Debug)]
pub struct BackBlastJsonData {
    pub ao: String,
    pub channel_id: String,
    pub q: Vec<String>,
    pub pax: Vec<String>,
    pub date: NaiveDate,
    pub bb_type: String,
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
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
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
