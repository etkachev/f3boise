use crate::app_state::ao_data::AO;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PublicChannels {
    BotPlayground,
    Backyard,
    Bleach,
    Gem,
    IronMountain,
    OldGlory,
    Rise,
    Rebel,
    Ruckership,
    DR,
    Welcome,
    LakeViewPark,
    KleinerPark,
    Unknown(String),
}

impl From<&AO> for PublicChannels {
    fn from(ao: &AO) -> Self {
        match ao {
            AO::Bleach => PublicChannels::Bleach,
            AO::Backyard => PublicChannels::Backyard,
            AO::Rise => PublicChannels::Rise,
            AO::Gem => PublicChannels::Gem,
            AO::OldGlory => PublicChannels::OldGlory,
            AO::Rebel => PublicChannels::Rebel,
            AO::IronMountain => PublicChannels::IronMountain,
            AO::Ruckership => PublicChannels::Ruckership,
            AO::LakeViewPark => PublicChannels::LakeViewPark,
            AO::KleinerPark => PublicChannels::KleinerPark,
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
            "ao-otb-bowler-park" | "ao-bowler-park" | "ao-rise" => PublicChannels::Rise,
            "ao-otb-lakeview-park" => PublicChannels::LakeViewPark,
            "ao-otb-kleiner-park" | "ao-otb-bellagio" => PublicChannels::KleinerPark,
            "ao-rebel" => PublicChannels::Rebel,
            "ao-ruckership" => PublicChannels::Ruckership,
            "downrange" => PublicChannels::DR,
            "welcome" => PublicChannels::Welcome,
            _ => PublicChannels::Unknown(name),
        }
    }
}
