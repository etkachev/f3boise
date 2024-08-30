use crate::shared::common_errors::AppError;
use crate::shared::time::convert_event_ts;
use crate::web_api_routes::reactions_log_data::ReactionLogRow;
use crate::web_api_routes::slack_events::emoji_reactions::ReactionData;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use uuid::Uuid;

pub struct ReactionLogDbItem {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub reaction: String,
    pub slack_user: String,
    pub reaction_added: bool,
    pub reaction_timestamp: NaiveDateTime,
}

impl ReactionLogDbItem {
    pub fn new(reaction_data: &ReactionData, entity_id: &str, added: bool) -> Self {
        ReactionLogDbItem {
            id: Uuid::new_v4(),
            entity_type: String::new(),
            entity_id: Uuid::from_str(entity_id).unwrap(),
            reaction: reaction_data.reaction.to_string(),
            slack_user: reaction_data.user.to_string(),
            reaction_added: added,
            reaction_timestamp: convert_event_ts(reaction_data.event_ts.as_str()).naive_utc(),
        }
    }

    pub fn for_pre_blast(mut self) -> Self {
        self.entity_type = String::from("pre_blast");
        self
    }
}

impl From<&ReactionLogRow> for ReactionLogDbItem {
    fn from(value: &ReactionLogRow) -> Self {
        ReactionLogDbItem {
            id: Uuid::from_str(value.id.as_str()).unwrap(),
            entity_id: Uuid::from_str(value.entity_id.as_str()).unwrap(),
            entity_type: value.entity_type.to_string(),
            reaction: value.reaction.to_string(),
            slack_user: value.slack_user.to_string(),
            reaction_added: value.reaction_added,
            reaction_timestamp: value.reaction_timestamp,
        }
    }
}

/// save single reaction item
pub async fn save_reaction_item(db: &PgPool, item: ReactionLogDbItem) -> Result<(), AppError> {
    let mut transaction = db.begin().await.expect("Failed to begin transaction");
    save_reaction_log_entry(&mut transaction, item).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

async fn save_reaction_log_entry(
    transaction: &mut Transaction<'_, Postgres>,
    item: ReactionLogDbItem,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    INSERT INTO reactions_log (id, entity_type, entity_id, reaction, slack_user, reaction_added, reaction_timestamp)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT ON CONSTRAINT reactions_log_slack_user_reaction_timestamp_key
        DO NOTHING;
    "#,
        item.id,
        item.entity_type,
        item.entity_id,
        item.reaction,
        item.slack_user,
        item.reaction_added,
        item.reaction_timestamp
    )
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

pub async fn sync_prod_from_rows(db: &PgPool, items: &[ReactionLogRow]) -> Result<(), AppError> {
    let mut transaction = db.begin().await.expect("Failed to begin transaction");
    for item in items.iter() {
        save_reaction_log_entry(&mut transaction, ReactionLogDbItem::from(item)).await?;
    }
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}
