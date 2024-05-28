use crate::app_state::backblast_data::BackBlastType;
use crate::db::queries::all_back_blasts::BackBlastJsonData;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use sqlx::PgPool;

/// Get back blasts by channel id
pub async fn back_blasts_by_channel_id(
    db_pool: &PgPool,
    channel_id: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    back_blasts_by_channel_id_and_type(db_pool, channel_id, BackBlastType::BackBlast).await
}

/// Get double downs by channel id
pub async fn double_downs_by_channel_id(
    db_pool: &PgPool,
    channel_id: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    back_blasts_by_channel_id_and_type(db_pool, channel_id, BackBlastType::DoubleDown).await
}

/// Get back blasts by channel id and type
async fn back_blasts_by_channel_id_and_type(
    db_pool: &PgPool,
    channel_id: &str,
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
            bb.channel_id,
            title
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = $1 AND bb.active = true AND bb.channel_id = $2
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type, title
    FROM list_view 
    ORDER BY date DESC;
    "#,
        bb_type,
        channel_id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}

/// get back blasts for a channel within date range
pub async fn back_blasts_by_channel_id_and_date_range(
    db_pool: &PgPool,
    channel_id: &str,
    date_range: (NaiveDate, NaiveDate),
) -> Result<Vec<BackBlastJsonData>, AppError> {
    back_blasts_by_channel_id_and_date_range_and_type(
        db_pool,
        channel_id,
        date_range,
        BackBlastType::BackBlast,
    )
    .await
}

/// get double downs for a channel within date range
pub async fn double_downs_by_channel_id_and_date_range(
    db_pool: &PgPool,
    channel_id: &str,
    date_range: (NaiveDate, NaiveDate),
) -> Result<Vec<BackBlastJsonData>, AppError> {
    back_blasts_by_channel_id_and_date_range_and_type(
        db_pool,
        channel_id,
        date_range,
        BackBlastType::DoubleDown,
    )
    .await
}

/// get back blasts for a channel within date range
async fn back_blasts_by_channel_id_and_date_range_and_type(
    db_pool: &PgPool,
    channel_id: &str,
    date_range: (NaiveDate, NaiveDate),
    bb_type: BackBlastType,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    let (start_date, end_date) = date_range;
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
            bb.channel_id,
            bb.title
        FROM back_blasts bb
        INNER JOIN ao_list al on bb.channel_id = al.channel_id
        WHERE bb.bb_type = $1 
            AND bb.active = true 
            AND bb.channel_id = $2
            AND bb.date >= $3
            AND bb.date <= $4
    )
    
    SELECT id, ao, channel_id, q as "q!", pax as "pax!", date, bb_type, title
    FROM list_view 
    ORDER BY date DESC;
    "#,
        bb_type,
        channel_id,
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}
