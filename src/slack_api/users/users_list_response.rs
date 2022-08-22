use crate::slack_api::cursor_data::CursorData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersListResponse {
    pub ok: bool,
    pub members: Option<Vec<SlackUserData>>,
    pub error: Option<String>,
    pub response_metadata: Option<CursorData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackUserData {
    pub id: String,
    pub name: String,
    pub profile: SlackUserProfile,
    pub deleted: bool,
    pub real_name: String,
    pub is_admin: bool,
    pub is_owner: bool,
    pub is_primary_owner: bool,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub is_bot: bool,
    pub is_app_user: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackUserProfile {
    pub title: String,
    pub phone: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
}
