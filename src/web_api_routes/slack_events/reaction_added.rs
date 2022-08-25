use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionAddedData {
    /// id of user who performed the reaction
    pub user: String,
    pub reaction: String,
    /// id of user that created original item that has been reacted to.
    pub item_user: String,
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

pub fn handle_reaction_item(reaction: &ReactionAddedData) {
    println!("Reaction: {:?}", reaction);
}
