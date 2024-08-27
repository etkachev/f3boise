use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::web_api_routes::slack_events::emoji_reactions::ReactionData;
use chrono::NaiveDateTime;
use sqlx::PgPool;
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
        let local_time = local_boise_time();
        ReactionLogDbItem {
            id: Uuid::new_v4(),
            entity_type: String::new(),
            entity_id: Uuid::from_str(entity_id).unwrap(),
            reaction: reaction_data.reaction.to_string(),
            slack_user: reaction_data.user.to_string(),
            reaction_added: added,
            reaction_timestamp: local_time.naive_utc(),
        }
    }

    pub fn for_pre_blast(mut self) -> Self {
        self.entity_type = String::from("pre_blast");
        self
    }
}

/// save single reaction item
pub async fn save_reaction_item(db: &PgPool, item: ReactionLogDbItem) -> Result<(), AppError> {
    sqlx::query!(
        r#"
    INSERT INTO reactions_log (id, entity_type, entity_id, reaction, slack_user, reaction_added, reaction_timestamp)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#,
        item.id,
        item.entity_type,
        item.entity_id,
        item.reaction,
        item.slack_user,
        item.reaction_added,
        item.reaction_timestamp
    )
        .execute(db)
        .await?;
    Ok(())
}
