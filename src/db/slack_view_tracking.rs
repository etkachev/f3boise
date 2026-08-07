use crate::shared::common_errors::AppError;
use sqlx::PgPool;

/// Check if a Slack view submission has already been processed
/// Returns Some(result_id) if already processed, None if not
pub async fn check_view_processed(
    db: &PgPool,
    view_id: &str,
) -> Result<Option<String>, AppError> {
    let result = sqlx::query!(
        r#"
        SELECT result_id
        FROM slack_view_submissions
        WHERE view_id = $1
        "#,
        view_id
    )
    .fetch_optional(db)
    .await?;

    Ok(result.map(|r| r.result_id))
}

/// Mark a Slack view submission as processed
/// This prevents duplicate processing if Slack retries the webhook
pub async fn mark_view_processed(
    db: &PgPool,
    view_id: &str,
    result_id: &str,
    view_type: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO slack_view_submissions (view_id, result_id, view_type)
        VALUES ($1, $2, $3)
        ON CONFLICT (view_id) DO NOTHING
        "#,
        view_id,
        result_id,
        view_type
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Clean up old processed view submissions (older than 1 hour)
/// This should be called periodically to prevent table bloat
pub async fn cleanup_old_views(db: &PgPool) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM slack_view_submissions
        WHERE expires_at < NOW()
        "#
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected())
}
