use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct BackBlastDate {
    pub ao: String,
    pub date: NaiveDate,
}

/// get backblast dates (only ao and date info) since a certain date.
pub async fn get_back_blasts_since(
    db_pool: &PgPool,
    date: &NaiveDate,
) -> Result<Vec<BackBlastDate>, AppError> {
    let rows: Vec<BackBlastDate> = sqlx::query_as!(
        BackBlastDate,
        r#"
        SELECT ao, date
        FROM back_blasts
        WHERE bb_type = 'backblast' AND active = true AND date >= $1
        ORDER BY date DESC;
        "#,
        date
    )
    .fetch_all(db_pool)
    .await?;
    Ok(rows)
}
