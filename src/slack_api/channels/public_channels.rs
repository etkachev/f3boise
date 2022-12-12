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
    RuckershipWest,
    RuckershipEast,
    WarHorse,
    Bellagio,
    Discovery,
    DR,
    Welcome,
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
            AO::RuckershipWest => PublicChannels::RuckershipWest,
            AO::RuckershipEast => PublicChannels::RuckershipEast,
            AO::WarHorse => PublicChannels::WarHorse,
            AO::Bellagio => PublicChannels::Bellagio,
            AO::Discovery => PublicChannels::Discovery,
            AO::DR => PublicChannels::DR,
            AO::Unknown(name) => PublicChannels::Unknown(name.to_string()),
        }
    }
}

impl From<String> for PublicChannels {
    // TODO use AO from
    fn from(name: String) -> Self {
        match name.as_str() {
            "bot-playground" => PublicChannels::BotPlayground,
            "ao-backyard" => PublicChannels::Backyard,
            "ao-bleach" => PublicChannels::Bleach,
            "ao-gem" => PublicChannels::Gem,
            "ao-iron-mountain" => PublicChannels::IronMountain,
            "ao-old-glory" => PublicChannels::OldGlory,
            "ao-otb-bowler-park" | "ao-bowler-park" | "ao-rise" => PublicChannels::Rise,
            "ao-otb-lakeview-park" | "ao-warhorse" => PublicChannels::WarHorse,
            "ao-otb-kleiner-park" | "ao-otb-bellagio" | "ao-bellagio" | "ao-bellagio-resort" => {
                PublicChannels::Bellagio
            }
            "ao-rebel" => PublicChannels::Rebel,
            "ao-ruckership-west" => PublicChannels::RuckershipWest,
            "ao-ruckership-east" => PublicChannels::RuckershipEast,
            "ao-discovery-park" => PublicChannels::Discovery,
            "downrange" => PublicChannels::DR,
            "welcome" => PublicChannels::Welcome,
            _ => PublicChannels::Unknown(name),
        }
    }
}
