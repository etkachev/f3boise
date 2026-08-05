use crate::app_state::MutableAppState;
use crate::db::queries::match_reaction_items::{get_items_by_ts_and_channel, ReactionRelatedItem};
use crate::db::save_reaction_log::{save_reaction_item, ReactionLogDbItem};
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
                    let reaction_log_item =
                        ReactionLogDbItem::new(reaction, &id, adding).for_pre_blast();
                    save_reaction_item(db, reaction_log_item).await?;

                    // If this is an HC reaction, notify F3
                    if reaction.reaction == "hc" {
                        if let Err(e) = notify_f3_hc(&id, &reaction.user, adding).await {
                            eprintln!("⚠️ Failed to notify F3 about HC: {}", e);
                            // Don't fail the whole operation if F3 notification fails
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Notify F3 when an HC reaction is added/removed on Slack
async fn notify_f3_hc(
    preblast_id: &str,
    slack_user_id: &str,
    adding: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get F3 webhook URL from environment variable
    let f3_webhook_url = std::env::var("F3_WEBHOOK_URL")
        .unwrap_or_else(|_| {
            eprintln!("⚠️ F3_WEBHOOK_URL not set, skipping F3 notification");
            String::new()
        });

    if f3_webhook_url.is_empty() {
        return Ok(());
    }

    let action = if adding { "add" } else { "remove" };
    let url = format!("{}/webhook/preblast/{}/hc", f3_webhook_url, preblast_id);

    println!(
        "🔔 [HC] Notifying F3: preblast={}, user={}, action={}",
        preblast_id, slack_user_id, action
    );

    #[derive(serde::Serialize)]
    struct HcWebhookPayload {
        slack_user_id: String,
        action: String,
    }

    let payload = HcWebhookPayload {
        slack_user_id: slack_user_id.to_string(),
        action: action.to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ [HC] Successfully notified F3");
    } else {
        eprintln!(
            "❌ [HC] F3 webhook failed: status={}, body={:?}",
            response.status(),
            response.text().await
        );
    }

    Ok(())
}
