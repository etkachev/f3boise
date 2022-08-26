use crate::slack_api::users::users_list::response::SlackUserData;
use crate::users::f3_user::F3User;
use std::collections::HashMap;

/// Basic bot info
#[derive(Debug)]
pub struct BotUser {
    pub id: String,
    pub name: String,
}

impl BotUser {
    pub fn from(user: &SlackUserData) -> Self {
        BotUser {
            id: user.id.to_string(),
            name: user.name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UserBotCombo {
    pub users: HashMap<String, F3User>,
    pub bots: HashMap<String, BotUser>,
}

impl UserBotCombo {
    pub fn new() -> Self {
        UserBotCombo {
            users: HashMap::new(),
            bots: HashMap::new(),
        }
    }
}

impl Default for UserBotCombo {
    fn default() -> Self {
        Self::new()
    }
}

/// name of this bot to check against.
pub const BOT_NAME: &str = "scraperrs";
