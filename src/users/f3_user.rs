use crate::slack_api::users::users_list::response::SlackUserData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct F3User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
}

impl F3User {
    pub fn non_slack_user(name: &str, email: &str) -> Self {
        F3User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    /// convert from reference of slack user data.
    pub fn from(user: &SlackUserData) -> Self {
        let cleaned_name = if let Some((name, _)) = user.profile.display_name.split_once('-') {
            name.trim()
        } else {
            user.profile.display_name.trim()
        };
        F3User {
            id: Some(user.id.to_string()),
            name: cleaned_name.to_string(),
            email: user
                .profile
                .email
                .as_ref()
                .map(|email| email.to_string())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
        }
    }
}
