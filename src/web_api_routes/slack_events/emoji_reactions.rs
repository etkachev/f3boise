use crate::app_state::MutableAppState;
use crate::db::queries::match_reaction_items::{get_items_by_ts_and_channel, ReactionRelatedItem};
use crate::db::queries::pre_blasts::get_pre_blast_by_id;
use crate::shared::common_errors::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionData {
    /// id of user who performed the reaction
    pub user: String,
    pub reaction: String,
    /// id of user that created original item that has been reacted to.
    pub item_user: Option<String>,
    /// reaction item itself.
    pub item: ReactionItem,
    pub event_ts: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionItem {
    #[serde(rename = "type")]
    pub event_type: String,
    /// public channel this message exists in.
    pub channel: String,
    /// time matches the timestamp for which message happened.
    pub ts: String,
}

pub async fn handle_reaction_add(
    db: &PgPool,
    reaction: &ReactionData,
    app_state: &MutableAppState,
) {
    if continue_with_emoji_handle(reaction, app_state) {
        println!("Reaction: {:?}", reaction);
        if let Err(err) = get_related_entity(db, reaction, true).await {
            println!("Error handling related entity: {:?}", err);
        }
    }
}

pub async fn handle_reaction_remove(
    db: &PgPool,
    reaction: &ReactionData,
    app_state: &MutableAppState,
) {
    if continue_with_emoji_handle(reaction, app_state) {
        println!("Removed reaction: {:?}", reaction);
        if let Err(err) = get_related_entity(db, reaction, false).await {
            println!("Error handling related entity: {:?}", err);
        }
    }
}

/// Don't listen to reactions from self.
fn continue_with_emoji_handle(reaction: &ReactionData, app_state: &MutableAppState) -> bool {
    let self_bot_id = {
        let app = app_state.app.lock().unwrap();
        app.self_bot_id.to_owned()
    };

    self_bot_id
        .map(|bot_id| reaction.user != bot_id)
        .unwrap_or(true)
}

/// match different related entities based on reaction data coming from slack
async fn get_related_entity(
    db: &PgPool,
    reaction: &ReactionData,
    adding: bool,
) -> Result<(), AppError> {
    // reactions on messages
    if reaction.item.event_type.as_str() == "message" {
        let data =
            get_items_by_ts_and_channel(db, &reaction.item.ts, &reaction.item.channel).await?;
        if let Some(data) = data {
            match data {
                ReactionRelatedItem::PreBlast(id) => {
                    let matching_pre_blast = get_pre_blast_by_id(db, &id).await?;
                    if let Some(pb) = matching_pre_blast {
                        let prefix = if adding {
                            "added reaction"
                        } else {
                            "removed reaction"
                        };
                        println!("{prefix} for matching preblast for: {}", pb.title);
                    }
                }
            }
        }
    }

    Ok(())
}
