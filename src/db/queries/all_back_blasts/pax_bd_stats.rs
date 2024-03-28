use crate::shared::common_errors::AppError;
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

pub async fn get_pax_bd_stats(db_pool: &PgPool) -> Result<Vec<PaxBdStats>, AppError> {
    let rows: Vec<PaxBdStats> = sqlx::query_as!(
        PaxBdStats,
        r#"
    WITH backblast_participants AS (
    SELECT
        bb.date,
        unnest(string_to_array(lower(pax), ',')) as user_name
    FROM
        back_blasts bb
    WHERE
        bb.bb_type = 'backblast' AND bb.active = true
),

     user_backblast_info AS (
         SELECT
             user_name,
             COUNT(*) AS bd_count,
             MIN(date) AS earliest_date
         FROM
             backblast_participants
         GROUP BY
             user_name
     )

SELECT
    u.slack_id,
    u.name,
    coalesce(ubi.bd_count, 0) as "bd_count!",
    ubi.earliest_date
FROM
    users u
        LEFT JOIN
    user_backblast_info ubi ON lower(u.name) = lower(ubi.user_name);
    "#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}

#[derive(Debug, Deserialize)]
pub struct PaxBdStats {
    pub slack_id: String,
    pub name: String,
    pub bd_count: i64,
    pub earliest_date: Option<NaiveDate>,
}
