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
pub mod double_downs;
pub mod equipment;
pub mod pre_blast_data;

pub struct MutableAppState {
    pub app: Mutex<AppState>,
}

impl MutableAppState {
    pub fn new() -> Self {
        MutableAppState {
            app: Mutex::new(AppState::default()),
        }
    }

    pub async fn sync_users(
        &self,
        db_pool: &PgPool,
        users: HashMap<String, F3User>,
    ) -> Result<(), AppError> {
        sync_users(db_pool, &users).await?;
        Ok(())
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

    pub fn get_channel_data(&self, channel: PublicChannels) -> Option<&ChannelData> {
        self.channels.get(&channel)
    }
}
