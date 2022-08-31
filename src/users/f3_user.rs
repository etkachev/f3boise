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
        let name = if user.profile.display_name.is_empty() {
            format!("{} {}", user.profile.first_name, user.profile.last_name)
        } else {
            user.profile.display_name.to_string()
        };
        let cleaned_name = if let Some((split_name, _)) = name.split_once('-') {
            split_name.trim()
        } else {
            name.trim()
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
