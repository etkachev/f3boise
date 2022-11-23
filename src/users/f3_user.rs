use crate::db::save_user::DbUser;
use crate::slack_api::users::users_list::response::SlackUserData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct F3User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
}

impl F3User {
    pub fn non_slack_user(name: &str, email: &str) -> Self {
        F3User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
            img_url: None,
        }
    }
}

impl From<&SlackUserData> for F3User {
    fn from(user: &SlackUserData) -> Self {
        let name = if user.profile.display_name.is_empty() {
            format!("{} {}", user.profile.first_name, user.profile.last_name)
        } else {
            user.profile.display_name.to_string()
        };
        let cleaned_name = if let Some((split_name, _)) = name.split_once(&['('][..]) {
            split_name.trim()
        } else {
            name.trim()
        };
        F3User {
            id: Some(user.id.to_string()),
            name: cleaned_name.to_string(),
            img_url: user.profile.image_24.clone(),
            email: user
                .profile
                .email
                .as_ref()
                .map(|email| email.to_string())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
        }
    }
}

impl From<DbUser> for F3User {
    fn from(user: DbUser) -> Self {
        F3User {
            id: Some(user.slack_id.to_string()),
            name: user.name.to_string(),
            email: user.email,
            img_url: user.img_url,
        }
    }
}
