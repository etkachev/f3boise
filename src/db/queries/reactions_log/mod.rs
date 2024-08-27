use crate::db::save_reaction_log::ReactionLogDbItem;
use crate::shared::common_errors::AppError;
use sqlx::PgPool;

/// download all reaction log items
pub async fn get_full_reaction_log(db: &PgPool) -> Result<Vec<ReactionLogDbItem>, AppError> {
    let results: Vec<ReactionLogDbItem> = sqlx::query_as!(
        ReactionLogDbItem,
        r#"
        SELECT id,
               entity_type,
               entity_id,
               reaction,
               slack_user,
               reaction_added,
               reaction_timestamp
        FROM reactions_log
        ORDER BY reaction_timestamp DESC;
    "#
    )
    .fetch_all(db)
    .await?;

    Ok(results)
}
