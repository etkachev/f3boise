use crate::app_state::MutableAppState;
use serde::{Deserialize, Serialize};

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

pub async fn handle_reaction_add(reaction: &ReactionData, app_state: &MutableAppState) {
    if continue_with_emoji_handle(reaction, app_state) {
        println!("Reaction: {:?}", reaction);
    }
}

pub async fn handle_reaction_remove(reaction: &ReactionData, app_state: &MutableAppState) {
    if continue_with_emoji_handle(reaction, app_state) {
        println!("Removed reaction: {:?}", reaction);
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
