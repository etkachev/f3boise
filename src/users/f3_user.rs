use crate::db::save_user::DbUser;
use crate::slack_api::users::users_list::response::SlackUserData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct F3User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
    pub invited_by: Option<F3Parent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum F3Parent {
    Pax(String),
    Online,
}

impl F3User {
    pub fn non_slack_user(name: &str, email: &str) -> Self {
        F3User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
            img_url: None,
            invited_by: None,
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
            img_url: user.profile.image_72.clone(),
            email: user
                .profile
                .email
                .as_ref()
                .map(|email| email.to_string())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
            invited_by: None,
        }
    }
}

impl From<DbUser> for F3User {
    fn from(user: DbUser) -> Self {
        let invited_by = user
            .parent_type
            .map(|pt| match pt.as_str() {
                "pax" => user.parent.map(|p| F3Parent::Pax(p.to_string())),
                "online" => Some(F3Parent::Online),
                _ => None,
            })
            .unwrap_or_default();

        F3User {
            id: Some(user.slack_id.to_string()),
            name: user.name.to_string(),
            email: user.email,
            img_url: user.img_url,
            invited_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_db_user_to_f3_user() {
        let db_user = DbUser {
            slack_id: "U67123".to_string(),
            name: "Stinger".to_string(),
            email: "edwardtkachev@gmail.com".to_string(),
            img_url: None,
            parent: Some("Canuck".to_string()),
            parent_type: Some("pax".to_string()),
        };

        let f3_user = F3User::from(db_user);

        assert_eq!(
            f3_user.invited_by.unwrap(),
            F3Parent::Pax("Canuck".to_string())
        );
    }
}
