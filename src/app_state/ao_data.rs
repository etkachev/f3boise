use crate::slack_api::channels::public_channels::PublicChannels;

/// Different AO options.
#[derive(PartialEq, Debug)]
pub enum AO {
    Bleach,
    Gem,
    OldGlory,
    Rebel,
    IronMountain,
    Ruckership,
    Backyard,
    BowlerPark,
    DR,
    Unknown(String),
}

impl From<String> for AO {
    fn from(ao: String) -> Self {
        let ao = ao.trim().to_lowercase();
        let split_text: Vec<&str> = ao.splitn(2, '#').collect();
        let cleaned_ao = if split_text.len() == 2 {
            split_text[1].to_lowercase()
        } else {
            ao.to_lowercase()
        };
        match cleaned_ao.as_str() {
            "bleach" => AO::Bleach,
            "gem" => AO::Gem,
            "old-glory" | "oldglory" => AO::OldGlory,
            "rebel" => AO::Rebel,
            "iron-mountain" | "ironmountain" => AO::IronMountain,
            "ruckership" | "rucker-ship" => AO::Ruckership,
            "bowler-park" => AO::BowlerPark,
            "backyard" => AO::Backyard,
            "dr" => AO::DR,
            _ => AO::Unknown(ao.to_string()),
        }
    }
}

impl From<PublicChannels> for AO {
    fn from(channel: PublicChannels) -> Self {
        match channel {
            PublicChannels::Rebel => AO::Rebel,
            PublicChannels::Ruckership => AO::Ruckership,
            PublicChannels::Backyard => AO::Backyard,
            PublicChannels::IronMountain => AO::IronMountain,
            PublicChannels::Bleach => AO::Bleach,
            PublicChannels::Gem => AO::Gem,
            PublicChannels::OldGlory => AO::OldGlory,
            PublicChannels::BowlerPark => AO::BowlerPark,
            PublicChannels::BotPlayground => AO::Unknown("BotPlayground".to_string()),
            PublicChannels::DR => AO::DR,
            PublicChannels::Unknown(unknown) => AO::Unknown(unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bleach_test() {
        let ao = AO::from("#bleach ".to_string());
        assert_eq!(ao, AO::Bleach);
        let ao = AO::from("#bleach\n".to_string());
        assert_eq!(ao, AO::Bleach);
    }
}
