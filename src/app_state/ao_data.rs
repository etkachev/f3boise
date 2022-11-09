use crate::slack_api::channels::public_channels::PublicChannels;
use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Different AO options.
#[derive(PartialEq, Debug, Deserialize, Serialize, Eq, Hash)]
pub enum AO {
    Bleach,
    Gem,
    OldGlory,
    Rebel,
    IronMountain,
    Ruckership,
    Backyard,
    Rise,
    WarHorse,
    Bellagio,
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
            AO::Rise => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::WarHorse => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::Bellagio => HashSet::from([Weekday::Tue, Weekday::Thu]),
            _ => HashSet::new(),
        }
    }

    /// whether or not AO is otb.
    pub fn is_otb(&self) -> bool {
        matches!(self, AO::WarHorse | AO::Bellagio)
    }

    pub fn channel_id(&self) -> &str {
        match self {
            AO::Bleach => const_names::BLEACH_CHANNEL_ID,
            AO::Gem => const_names::GEM_CHANNEL_ID,
            AO::OldGlory => const_names::OLD_GLORY_CHANNEL_ID,
            AO::Rebel => const_names::REBEL_CHANNEL_ID,
            AO::IronMountain => const_names::IRON_MOUNTAIN_CHANNEL_ID,
            AO::Ruckership => const_names::RUCKERSHIP_CHANNEL_ID,
            AO::Backyard => const_names::BACKYARD_CHANNEL_ID,
            AO::Rise => const_names::RISE_CHANNEL_ID,
            AO::WarHorse => const_names::WAR_HORSE_CHANNEL_ID,
            AO::Bellagio => const_names::BELLAGIO_CHANNEL_ID,
            AO::DR => const_names::DR_CHANNEL_ID,
            AO::Unknown(_) => "UNKNOWN",
        }
    }

    pub fn from_channel_id(channel_id: &str) -> Self {
        match channel_id {
            const_names::BLEACH_CHANNEL_ID => AO::Bleach,
            const_names::GEM_CHANNEL_ID => AO::Gem,
            const_names::OLD_GLORY_CHANNEL_ID => AO::OldGlory,
            const_names::REBEL_CHANNEL_ID => AO::Rebel,
            const_names::IRON_MOUNTAIN_CHANNEL_ID => AO::IronMountain,
            const_names::RUCKERSHIP_CHANNEL_ID => AO::Ruckership,
            const_names::BACKYARD_CHANNEL_ID => AO::Backyard,
            const_names::RISE_CHANNEL_ID => AO::Rise,
            const_names::WAR_HORSE_CHANNEL_ID => AO::WarHorse,
            const_names::BELLAGIO_CHANNEL_ID => AO::Bellagio,
            const_names::DR_CHANNEL_ID => AO::DR,
            _ => AO::Unknown("UNKNOWN".to_string()),
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
            AO::Rise => AO::Rise,
            AO::DR => AO::DR,
            AO::WarHorse => AO::WarHorse,
            AO::Bellagio => AO::Bellagio,
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
            AO::Rise => const_names::RISE,
            AO::WarHorse => const_names::WAR_HORSE,
            AO::Bellagio => const_names::BELLAGIO,
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

        let cleaned_ao = cleaned_ao
            .strip_prefix("ao-")
            .unwrap_or(cleaned_ao.as_str());

        match cleaned_ao {
            const_names::BLEACH => AO::Bleach,
            const_names::GEM => AO::Gem,
            const_names::OLD_GLORY | "oldglory" => AO::OldGlory,
            const_names::REBEL => AO::Rebel,
            const_names::IRON_MOUNTAIN | "ironmountain" => AO::IronMountain,
            const_names::RUCKERSHIP | "rucker-ship" => AO::Ruckership,
            const_names::RISE | "bowler-park" => AO::Rise,
            const_names::WAR_HORSE | "lakeview_park" => AO::WarHorse,
            const_names::BELLAGIO => AO::Bellagio,
            const_names::BACKYARD => AO::Backyard,
            const_names::DR => AO::DR,
            _ => AO::Unknown(ao.to_string()),
        }
    }
}

impl From<PublicChannels> for AO {
    fn from(channel: PublicChannels) -> Self {
        channel_to_ao_mapper(&channel)
    }
}

impl From<&PublicChannels> for AO {
    fn from(channel: &PublicChannels) -> Self {
        channel_to_ao_mapper(channel)
    }
}

/// shared mapper from public channel to ao
fn channel_to_ao_mapper(channel: &PublicChannels) -> AO {
    match channel {
        PublicChannels::Rebel => AO::Rebel,
        PublicChannels::Ruckership => AO::Ruckership,
        PublicChannels::Backyard => AO::Backyard,
        PublicChannels::IronMountain => AO::IronMountain,
        PublicChannels::Bleach => AO::Bleach,
        PublicChannels::Gem => AO::Gem,
        PublicChannels::OldGlory => AO::OldGlory,
        PublicChannels::Rise => AO::Rise,
        PublicChannels::WarHorse => AO::WarHorse,
        PublicChannels::Bellagio => AO::Bellagio,
        PublicChannels::BotPlayground => AO::Unknown("BotPlayground".to_string()),
        PublicChannels::DR => AO::DR,
        PublicChannels::Welcome => AO::Unknown("Welcome".to_string()),
        PublicChannels::Unknown(unknown) => AO::Unknown(unknown.to_string()),
    }
}

/// represents ao for db
#[derive(Debug, Deserialize, Serialize)]
pub struct AoData {
    /// name of AO
    pub name: String,
    /// should be comma separated list of days of week this AO meets
    pub days: String,
    /// whether it's an official AO or otb (Off the books)
    pub active: bool,
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
            active: !ao.is_otb(),
        }
    }
}

pub mod const_names {
    use super::AO;

    pub const BLEACH: &str = "bleach";
    pub const BLEACH_CHANNEL_ID: &str = "C03UR7GM7Q9";
    pub const GEM: &str = "gem";
    pub const GEM_CHANNEL_ID: &str = "C03UBFXVBGD";
    pub const OLD_GLORY: &str = "old-glory";
    pub const OLD_GLORY_CHANNEL_ID: &str = "C03TZTPUFRV";
    pub const REBEL: &str = "rebel";
    pub const REBEL_CHANNEL_ID: &str = "C03V463RFRN";
    pub const IRON_MOUNTAIN: &str = "iron-mountain";
    pub const IRON_MOUNTAIN_CHANNEL_ID: &str = "C03TZTTHDPZ";
    pub const RUCKERSHIP: &str = "ruckership";
    pub const RUCKERSHIP_CHANNEL_ID: &str = "C03V46DGXMW";
    pub const BACKYARD: &str = "backyard";
    pub const BACKYARD_CHANNEL_ID: &str = "C03UEBT1QRZ";
    pub const RISE: &str = "rise";
    pub const RISE_CHANNEL_ID: &str = "C03UT46303T";
    /// TODO otb
    pub const WAR_HORSE: &str = "warhorse";
    pub const WAR_HORSE_CHANNEL_ID: &str = "C0425DL9MT7";
    pub const BELLAGIO: &str = "bellagio";
    pub const BELLAGIO_CHANNEL_ID: &str = "C045SMRL43X";
    pub const DR: &str = "dr";
    pub const DR_CHANNEL_ID: &str = "C03U7U9T7HU";

    /// full list of active aos
    pub const AO_LIST: [AO; 10] = [
        AO::Bleach,
        AO::Gem,
        AO::OldGlory,
        AO::Rebel,
        AO::IronMountain,
        AO::Ruckership,
        AO::Backyard,
        AO::Rise,
        AO::WarHorse,
        AO::Bellagio,
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

    #[test]
    fn gem_test() {
        let ao = AO::from("gem".to_string());
        assert_eq!(ao, AO::Gem);
    }

    #[test]
    fn warhorse() {
        let ao = AO::from("#ao-warhorse".to_string());
        assert_eq!(ao, AO::WarHorse);
    }
}
