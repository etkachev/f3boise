use crate::slack_api::channels::public_channels::PublicChannels;
use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

/// days of the week the ao is open
pub type AoDays = HashSet<Weekday>;

impl AO {
    pub fn week_days(&self) -> AoDays {
        match self {
            AO::Bleach => HashSet::from([Weekday::Mon, Weekday::Wed, Weekday::Sat]),
            AO::Gem => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::OldGlory => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::Rebel => HashSet::from([Weekday::Tue, Weekday::Thu]),
            AO::IronMountain => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::Ruckership => HashSet::from([Weekday::Fri]),
            AO::Backyard => HashSet::from([Weekday::Wed, Weekday::Fri]),
            AO::BowlerPark => HashSet::from([Weekday::Mon, Weekday::Wed]),
            _ => HashSet::new(),
        }
    }
}

impl Clone for AO {
    fn clone(&self) -> Self {
        match self {
            AO::Bleach => AO::Bleach,
            AO::Gem => AO::Gem,
            AO::OldGlory => AO::OldGlory,
            AO::Rebel => AO::Rebel,
            AO::IronMountain => AO::IronMountain,
            AO::Ruckership => AO::Ruckership,
            AO::Backyard => AO::Backyard,
            AO::BowlerPark => AO::BowlerPark,
            AO::DR => AO::DR,
            AO::Unknown(name) => AO::Unknown(name.to_string()),
        }
    }
}

impl ToString for AO {
    fn to_string(&self) -> String {
        let name = match self {
            AO::Bleach => const_names::BLEACH,
            AO::Gem => const_names::GEM,
            AO::OldGlory => const_names::OLD_GLORY,
            AO::Rebel => const_names::REBEL,
            AO::IronMountain => const_names::IRON_MOUNTAIN,
            AO::Ruckership => const_names::RUCKERSHIP,
            AO::Backyard => const_names::BACKYARD,
            AO::BowlerPark => const_names::BOWLER_PARK,
            AO::DR => "",
            AO::Unknown(_) => "",
        };
        name.to_string()
    }
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
            const_names::BLEACH => AO::Bleach,
            const_names::GEM => AO::Gem,
            const_names::OLD_GLORY | "oldglory" => AO::OldGlory,
            const_names::REBEL => AO::Rebel,
            const_names::IRON_MOUNTAIN | "ironmountain" => AO::IronMountain,
            const_names::RUCKERSHIP | "rucker-ship" => AO::Ruckership,
            const_names::BOWLER_PARK => AO::BowlerPark,
            const_names::BACKYARD => AO::Backyard,
            const_names::DR => AO::DR,
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

/// represents ao for db
#[derive(Debug, Deserialize, Serialize)]
pub struct AoData {
    /// name of AO
    pub name: String,
    /// should be comma separated list of days of week this AO meets
    pub days: String,
}

impl AoData {
    pub fn from(ao: &AO) -> Self {
        let days = ao.week_days();
        let serialized = days
            .into_iter()
            .map(|day| day.to_string())
            .collect::<Vec<String>>()
            .join(",");
        AoData {
            name: ao.to_string(),
            days: serialized,
        }
    }
}

pub mod const_names {
    use super::AO;

    pub const BLEACH: &str = "bleach";
    pub const GEM: &str = "gem";
    pub const OLD_GLORY: &str = "old-glory";
    pub const REBEL: &str = "rebel";
    pub const IRON_MOUNTAIN: &str = "iron-mountain";
    pub const RUCKERSHIP: &str = "ruckership";
    pub const BACKYARD: &str = "backyard";
    pub const BOWLER_PARK: &str = "bowler-park";
    pub const DR: &str = "dr";

    /// full list of active aos
    pub const AO_LIST: [AO; 8] = [
        AO::Bleach,
        AO::Gem,
        AO::OldGlory,
        AO::Rebel,
        AO::IronMountain,
        AO::Ruckership,
        AO::Backyard,
        AO::BowlerPark,
    ];
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
