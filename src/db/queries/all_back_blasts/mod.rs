use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

/// get all back blast data (with type 'backblast')
pub async fn get_all(db_pool: &PgPool) -> Result<Vec<BackBlastJsonData>, AppError> {
    let rows: Vec<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            ao,
            string_to_array(q, ',') as q,
            string_to_array(pax, ',') as pax,
            date,
            bb_type
        FROM back_blasts
    )
    
    SELECT ao, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    WHERE bb_type = 'backblast'
    ORDER BY date DESC;
    "#
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

#[derive(Deserialize)]
pub struct BackBlastJsonData {
    pub ao: String,
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
    let rows: Vec<BackBlastJsonData> = sqlx::query_as!(
        BackBlastJsonData,
        r#"
    WITH list_view AS (
        SELECT
            ao,
            string_to_array(q, ',') as q,
            string_to_array(pax, ',') as pax,
            date,
            bb_type
        FROM back_blasts
    )
    
    SELECT ao, q as "q!", pax as "pax!", date, bb_type
    FROM list_view 
    WHERE pax @> array[$1] AND bb_type = 'backblast'
    ORDER BY date DESC;
    "#,
        name
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}
