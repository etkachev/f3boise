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
