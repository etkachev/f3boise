use crate::slack_api::cursor_data::CursorData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsHistoryResponse {
    pub ok: bool,
    pub messages: Option<Vec<MessageData>>,
    pub error: Option<String>,
    pub has_more: Option<bool>,
    pub pin_count: Option<u16>,
    pub response_metadata: Option<CursorData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData {
    // full raw string,
    pub text: String,
    // timestamp
    pub ts: String,
    // user id that posted.
    pub user: String,
    pub reactions: Option<Vec<MessageReaction>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: u16,
}
