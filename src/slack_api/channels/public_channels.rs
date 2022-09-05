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

impl From<&AO> for PublicChannels {
    fn from(ao: &AO) -> Self {
        match ao {
            AO::Bleach => PublicChannels::Bleach,
            AO::Backyard => PublicChannels::Backyard,
            AO::BowlerPark => PublicChannels::BowlerPark,
            AO::Gem => PublicChannels::Gem,
            AO::OldGlory => PublicChannels::OldGlory,
            AO::Rebel => PublicChannels::Rebel,
            AO::IronMountain => PublicChannels::IronMountain,
            AO::Ruckership => PublicChannels::Ruckership,
            AO::DR => PublicChannels::DR,
            AO::Unknown(name) => PublicChannels::Unknown(name.to_string()),
        }
    }
}

impl From<String> for PublicChannels {
    fn from(name: String) -> Self {
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
