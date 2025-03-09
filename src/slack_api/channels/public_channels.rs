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
    Tower,
    BlackDiamond,
    BlackOps,
    CamelsBack,
    Coop,
    GooseDynasty,
    DarkStride,
    Interceptor,
    MallardPark,
    BlackCanyon,
    Liberty,
    FirstF,
    DR,
    Welcome,
    HelpDesk,
    MumbleChatter,
    Unknown(String),
}

impl PublicChannels {
    pub fn channel_id(&self) -> String {
        let ao = AO::from(self);
        match self {
            PublicChannels::MumbleChatter => "C03SZ3J3SB0",
            PublicChannels::HelpDesk => "C03T2ND0YF7",
            PublicChannels::Welcome => "C03T2Q7U337",

            _ => ao.channel_id(),
        }
        .to_string()
    }
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
            AO::Tower => PublicChannels::Tower,
            AO::BlackDiamond => PublicChannels::BlackDiamond,
            AO::BlackOps => PublicChannels::BlackOps,
            AO::FirstF => PublicChannels::FirstF,
            AO::CamelsBack => PublicChannels::CamelsBack,
            AO::Coop => PublicChannels::Coop,
            AO::GooseDynasty => PublicChannels::GooseDynasty,
            AO::DarkStride => PublicChannels::DarkStride,
            AO::Interceptor => PublicChannels::Interceptor,
            AO::MallardPark => PublicChannels::MallardPark,
            AO::BlackCanyon => PublicChannels::BlackCanyon,
            AO::Liberty => PublicChannels::Liberty,
            AO::DR => PublicChannels::DR,
            AO::Unknown(name) => PublicChannels::Unknown(name.to_string()),
        }
    }
}

impl From<String> for PublicChannels {
    fn from(name: String) -> Self {
        match name.as_str() {
            "downrange" => PublicChannels::DR,
            "welcome" => PublicChannels::Welcome,
            "bot-playground" => PublicChannels::BotPlayground,
            "ao-bellagio" | "ao-bellagio-resort" => PublicChannels::Bellagio,
            _ => {
                let ao = AO::from(name);
                PublicChannels::from(&ao)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_channel_name(name: &str, expected: PublicChannels) {
        let channel = PublicChannels::from(name.to_string());
        assert_eq!(expected, channel);
    }

    #[test]
    fn test_all() {
        test_channel_name("downrange", PublicChannels::DR);
        test_channel_name("ao-bleach", PublicChannels::Bleach);
        test_channel_name("ao-backyard", PublicChannels::Backyard);
        test_channel_name("ao-gem", PublicChannels::Gem);
        test_channel_name("ao-iron-mountain", PublicChannels::IronMountain);
        test_channel_name("ao-old-glory", PublicChannels::OldGlory);
        test_channel_name("ao-rise", PublicChannels::Rise);
        test_channel_name("ao-warhorse", PublicChannels::WarHorse);
        test_channel_name("ao-bellagio-resort", PublicChannels::Bellagio);
        test_channel_name("ao-bellagio", PublicChannels::Bellagio);
        test_channel_name("ao-rebel", PublicChannels::Rebel);
        test_channel_name("ao-ruckership-west", PublicChannels::RuckershipWest);
        test_channel_name("ao-ruckership-east", PublicChannels::RuckershipEast);
        test_channel_name("ao-tower", PublicChannels::Tower);
        test_channel_name("ao-black-diamond", PublicChannels::BlackDiamond);
        test_channel_name("ao-black-ops", PublicChannels::BlackOps);
        test_channel_name("black-ops", PublicChannels::BlackOps);
        test_channel_name("camels-back", PublicChannels::CamelsBack);
        test_channel_name("ao-coop", PublicChannels::Coop);
        test_channel_name("otb-goose-dynasty", PublicChannels::GooseDynasty);
        test_channel_name("goose-dynasty", PublicChannels::GooseDynasty);
        test_channel_name("ao-dark-stride", PublicChannels::DarkStride);
        test_channel_name("otb-mallard-park", PublicChannels::MallardPark);
        test_channel_name("ao-black-canyon", PublicChannels::BlackCanyon);
        test_channel_name("1st-f", PublicChannels::FirstF);
    }

    #[test]
    fn string_to_channel_bleach() {
        let name = "ao-bleach".to_string();
        let channel = PublicChannels::from(name);
        assert_eq!(PublicChannels::Bleach, channel);
    }

    #[test]
    fn string_to_channel_bot_playground() {
        let name = "bot-playground".to_string();
        let channel = PublicChannels::from(name);
        assert_eq!(PublicChannels::BotPlayground, channel);
    }

    #[test]
    fn string_fake_channel() {
        let channel = PublicChannels::from("fake".to_string());
        assert_eq!(PublicChannels::Unknown("fake".to_string()), channel);
    }
}
