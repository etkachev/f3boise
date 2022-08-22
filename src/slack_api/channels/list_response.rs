use crate::slack_api::cursor_data::CursorData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsListResponse {
    pub ok: bool,
    pub channels: Option<Vec<ChannelData>>,
    pub response_metadata: Option<CursorData>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelData {
    pub id: String,
    pub name: String,
}
