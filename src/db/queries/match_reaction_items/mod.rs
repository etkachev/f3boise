//! This is for queries matching different items in database to connect any emoji or reactions we want to track

use crate::shared::common_errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

/// data item in db that could be tied to a reaction/emoji
pub enum ReactionRelatedItem {
    /// returns id of preblast
    PreBlast(String),
}

struct ReactionRelatedDbItem {
    id: Uuid,
    item_type: String,
}

impl TryFrom<ReactionRelatedDbItem> for ReactionRelatedItem {
    type Error = AppError;

    fn try_from(value: ReactionRelatedDbItem) -> Result<Self, Self::Error> {
        match value.item_type.as_str() {
            "pre-blast" => Ok(ReactionRelatedItem::PreBlast(value.id.to_string())),
            _ => Err(AppError::from("Couldn't match on reaction related item")),
        }
    }
}

/// get matching db item by timestamp of message and channel id.
/// Could be preblast, etc
pub async fn get_items_by_ts_and_channel(
    db: &PgPool,
    ts: &str,
    channel_id: &str,
) -> Result<Option<ReactionRelatedItem>, AppError> {
    let result: Option<ReactionRelatedDbItem> = sqlx::query_as!(
        ReactionRelatedDbItem,
        r#"
        SELECT 'pre-blast' as "item_type!", id
        FROM pre_blasts
        WHERE ts = $1 AND channel_id = $2;
    "#,
        ts,
        channel_id
    )
    .fetch_optional(db)
    .await?;

    let result = result
        .map(|item| ReactionRelatedItem::try_from(item).ok())
        .unwrap_or_default();

    Ok(result)
}
