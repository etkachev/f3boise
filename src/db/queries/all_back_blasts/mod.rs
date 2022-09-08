use crate::db::save_back_blast::BackBlastDbEntry;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use sqlx::types::JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

/// get all back blast data (with type 'backblast')
pub async fn get_all(db_pool: &PgPool) -> Result<Vec<BackBlastDbEntry>, AppError> {
    let rows: Vec<BackBlastDbEntry> = sqlx::query_as!(
        BackBlastDbEntry,
        r#"
    SELECT id, ao, q, pax, date, bb_type
    FROM back_blasts
    WHERE bb_type = 'backblast'
    ORDER BY date DESC;
    "#
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

pub struct BackBlastJsonData {
    pub id: Uuid,
    pub ao: String,
    pub q: Option<JsonValue>,
    pub pax: Option<JsonValue>,
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
            id,
            ao,
            to_jsonb(string_to_array(q, ',')) as q,
            to_jsonb(string_to_array(pax, ',')) as pax,
            date,
            bb_type
        FROM back_blasts
    )
    
    SELECT id, ao, q, pax, date, bb_type
    FROM list_view 
    WHERE pax ?| array[$1] AND bb_type = 'backblast'
    ORDER BY date DESC;
    "#,
        name
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}
