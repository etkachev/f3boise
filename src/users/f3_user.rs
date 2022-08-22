use crate::slack_api::users::users_list_response::SlackUserData;

#[derive(Debug)]
pub struct F3User {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<SlackUserData> for F3User {
    fn from(user: SlackUserData) -> Self {
        let cleaned_name = if let Some((name, _)) = user.profile.display_name.split_once('-') {
            name.trim()
        } else {
            user.profile.display_name.trim()
        };
        F3User {
            id: user.id,
            name: cleaned_name.to_string(),
            email: user.profile.email.unwrap_or_else(|| "UNKNOWN".to_string()),
        }
    }
}
