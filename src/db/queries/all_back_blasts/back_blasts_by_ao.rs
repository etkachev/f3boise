use crate::db::queries::all_back_blasts::BackBlastJsonData;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use sqlx::PgPool;

/// Get back blasts by channel id
pub async fn back_blasts_by_channel_id(
    db_pool: &PgPool,
    channel_id: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
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
        WHERE bb.bb_type = 'backblast' AND bb.active = true AND bb.channel_id = $1
    )
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#,
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
    let (start_date, end_date) = date_range;
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
        WHERE bb.bb_type = 'backblast' 
            AND bb.active = true 
            AND bb.channel_id = $1
            AND bb.date >= $2
            AND bb.date <= $3
    )
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#,
        channel_id,
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}
