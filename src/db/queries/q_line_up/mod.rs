use crate::app_state::ao_data::AO;
use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct QLineUpDbData {
    pub ao: String,
    pub qs: Vec<String>,
    pub date: NaiveDate,
    pub closed: bool,
}

/// get single q line up if exists based on ao and date
pub async fn get_single_q_line_up(
    db_pool: &PgPool,
    date: &NaiveDate,
    channel_id: &str,
) -> Result<Option<QLineUpDbData>, AppError> {
    let item: Option<QLineUpDbData> = sqlx::query_as!(
        QLineUpDbData,
        r#"
        WITH list_view AS (
            SELECT
                al.name as ao,
                string_to_array(lower(qlu.qs), ',') as qs,
                qlu.date,
                qlu.closed
            FROM q_line_up qlu
                INNER JOIN ao_list al on qlu.channel_id = al.channel_id
            WHERE al.channel_id = $2
        )
        
    SELECT ao, qs as "qs!", date, closed
    FROM list_view
    WHERE date = $1;
    "#,
        date,
        channel_id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(item)
}

/// Get q line up between 2 dates
pub async fn get_q_line_up_between_dates(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<QLineUpDbData>, AppError> {
    let rows: Vec<QLineUpDbData> = sqlx::query_as!(
        QLineUpDbData,
        r#"
        WITH list_view AS (
            SELECT
                al.name as ao,
                string_to_array(lower(qlu.qs), ',') as qs,
                qlu.date,
                qlu.closed
            FROM q_line_up qlu
                INNER JOIN ao_list al on qlu.channel_id = al.channel_id
        )
        
        SELECT ao, qs as "qs!", date, closed
        FROM list_view
        WHERE date BETWEEN $1 AND $2
        ORDER BY date ASC;
        "#,
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// Get q line up between 2 dates for an AO
pub async fn get_q_line_up_between_dates_for_ao(
    db_pool: &PgPool,
    ao: &AO,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<QLineUpDbData>, AppError> {
    let rows: Vec<QLineUpDbData> = sqlx::query_as!(
        QLineUpDbData,
        r#"
        WITH list_view AS (
            SELECT
                al.name as ao,
                string_to_array(lower(qlu.qs), ',') as qs,
                qlu.date,
                qlu.closed
            FROM q_line_up qlu
                INNER JOIN ao_list al on qlu.channel_id = al.channel_id
        )
        
        SELECT ao, qs as "qs!", date, closed
        FROM list_view
        WHERE ao = $1 AND date BETWEEN $2 AND $3
        ORDER BY date ASC;
        "#,
        ao.to_string(),
        start_date,
        end_date
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

/// get all q line up db items for syncing
pub async fn get_all_q_line_up_db_items(
    db_pool: &PgPool,
) -> Result<Vec<RawQLineUpDbEntry>, AppError> {
    let rows: Vec<RawQLineUpDbEntry> = sqlx::query_as!(
        RawQLineUpDbEntry,
        r#"
        SELECT qs, ao, date, closed, channel_id
        FROM q_line_up
        ORDER BY date ASC;
        "#,
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}

#[derive(Deserialize)]
pub struct RawQLineUpDbEntry {
    pub qs: String,
    pub ao: String,
    pub date: NaiveDate,
    pub closed: bool,
    pub channel_id: String,
}
