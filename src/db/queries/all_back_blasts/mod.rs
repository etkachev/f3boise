use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// get all back blast data (with type 'backblast')
pub async fn get_all(db_pool: &PgPool) -> Result<Vec<BackBlastJsonData>, AppError> {
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
        WHERE bb.bb_type = 'backblast' AND bb.active = true
    )
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// get all back_blasts within an ao range
pub async fn get_all_within_date_range(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
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
        WHERE bb.bb_type = 'backblast' AND bb.active = true AND bb.date >= $1 AND bb.date <= $2
    )
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    ORDER BY date DESC;
    "#,
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

#[derive(Deserialize, Serialize)]
pub struct BackBlastJsonData {
    pub ao: String,
    pub channel_id: String,
    pub q: Vec<String>,
    pub pax: Vec<String>,
    pub date: NaiveDate,
    pub bb_type: String,
}

/// get list test
pub async fn get_list_with_pax(
    db_pool: &PgPool,
    name: &str,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    let name = name.to_lowercase();
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
        WHERE bb.bb_type = 'backblast' AND bb.active = true
    )
    
    SELECT ao, channel_id, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    WHERE pax @> array[$1]
    ORDER BY date DESC;
    "#,
        name
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}
