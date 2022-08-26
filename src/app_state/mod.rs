use crate::bot_data::{BotUser, BOT_NAME};
use crate::slack_api::channels::{list::response::ChannelData, public_channels::PublicChannels};
use crate::users::f3_user::F3User;
use std::collections::HashMap;

pub mod ao_data;
pub mod backblast_data;
pub mod parse_backblast;

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

    pub fn add_user(&mut self, name: &str, user: F3User) {
        self.users.insert(name.to_string(), user);
    }

    pub fn get_channel_data(&self, channel: PublicChannels) -> Option<&ChannelData> {
        self.channels.get(&channel)
    }
}
