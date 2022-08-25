use crate::app_state::ao_data::AO;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PublicChannels {
    BotPlayground,
    Backyard,
    Bleach,
    Gem,
    IronMountain,
    OldGlory,
    BowlerPark,
    Rebel,
    Ruckership,
    DR,
    Unknown(String),
}

impl PublicChannels {
    pub fn from(ao: &AO) -> Self {
        match ao {
            // TODO for now just forward to bot
            _ => PublicChannels::BotPlayground,
        }
    }

    pub fn from_name(name: String) -> Self {
        match name.as_str() {
            "bot-playground" => PublicChannels::BotPlayground,
            "ao-backyard" => PublicChannels::Backyard,
            "ao-bleach" => PublicChannels::Bleach,
            "ao-gem" => PublicChannels::Gem,
            "ao-iron-mountain" => PublicChannels::IronMountain,
            "ao-old-glory" => PublicChannels::OldGlory,
            "ao-otb-bowler-park" | "ao-bowler-park" => PublicChannels::BowlerPark,
            "ao-rebel" => PublicChannels::Rebel,
            "ao-ruckership" => PublicChannels::Ruckership,
            "downrange" => PublicChannels::DR,
            _ => PublicChannels::Unknown(name),
        }
    }
}
