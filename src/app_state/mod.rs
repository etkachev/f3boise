use crate::bot_data::{BotUser, BOT_NAME};
use crate::db::init::sync_users;
use crate::shared::common_errors::AppError;
use crate::slack_api::channels::{list::response::ChannelData, public_channels::PublicChannels};
use crate::users::f3_user::F3User;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod ao_data;
pub mod backblast_data;
pub mod parse_backblast;

pub struct MutableAppState {
    pub app: Mutex<AppState>,
}

impl MutableAppState {
    pub fn new() -> Self {
        MutableAppState {
            app: Mutex::new(AppState::default()),
        }
    }

    pub async fn sync_users(&self, db_pool: &PgPool) -> Result<(), AppError> {
        let users = {
            let app = self.app.lock().expect("Could not lock app state");
            app.users.clone()
        };
        sync_users(db_pool, &users).await?;
        Ok(())
    }

    pub fn insert_users(&mut self, users: HashMap<String, F3User>) {
        let mut app = self.app.lock().expect("Could not lock app state");
        app.users.extend(users);
    }

    pub fn insert_bots(&mut self, bots: HashMap<String, BotUser>) {
        let mut app = self.app.lock().expect("Could not lock app state");
        app.bots = bots;
        app.set_self_bot_id();
    }

    pub fn insert_channels(&mut self, channels: HashMap<PublicChannels, ChannelData>) {
        let mut app = self.app.lock().expect("Could not lock app state");
        app.channels = channels;
    }
}

impl Default for MutableAppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub channels: HashMap<PublicChannels, ChannelData>,
    pub users: HashMap<String, F3User>,
    pub bots: HashMap<String, BotUser>,
    /// id of the bot this app is.
    pub self_bot_id: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            ..Default::default()
        }
    }

    pub fn set_self_bot_id(&mut self) {
        let matched_bot = self.bots.iter().find_map(|(id, bot)| {
            if bot.name == BOT_NAME {
                Some(id.to_string())
            } else {
                None
            }
        });
        self.self_bot_id = matched_bot;
    }

    pub fn add_user(&mut self, id: &str, user: F3User) {
        self.users.insert(id.to_string(), user);
    }

    pub fn get_channel_data(&self, channel: PublicChannels) -> Option<&ChannelData> {
        self.channels.get(&channel)
    }

    /// get hashmap where key is slack id and value is f3 name
    pub fn get_slack_id_map(&self) -> HashMap<String, String> {
        self.users
            .iter()
            .fold(HashMap::<String, String>::new(), |mut acc, (id, user)| {
                acc.insert(id.to_string(), user.name.to_lowercase());
                acc
            })
    }

    /// get hashmap where key is f3 name and value is slack id
    pub fn get_user_name_map(&self) -> HashMap<String, String> {
        self.users
            .iter()
            .fold(HashMap::<String, String>::new(), |mut acc, (id, user)| {
                acc.insert(user.name.to_lowercase(), id.to_string());
                acc
            })
    }
}
