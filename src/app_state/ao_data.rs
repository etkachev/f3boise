use crate::app_state::equipment::AoEquipment;
use crate::slack_api::channels::public_channels::PublicChannels;
use chrono::{Datelike, NaiveDate, NaiveTime, Weekday};
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
    RuckershipWest,
    RuckershipEast,
    Backyard,
    Rise,
    WarHorse,
    Bellagio,
    Discovery,
    BlackDiamond,
    BlackOps,
    DR,
    Unknown(String),
}

/// days of the week the ao is open
pub type AoDays = HashSet<Weekday>;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum AoType {
    Bootcamp,
    Heavy,
    Running,
    Rucking,
}

impl ToString for AoType {
    fn to_string(&self) -> String {
        match self {
            AoType::Bootcamp => String::from("Bootcamp"),
            AoType::Heavy => String::from("Ruck/Sandbag"),
            AoType::Running => String::from("Running"),
            AoType::Rucking => String::from("Ruck/Hike"),
        }
    }
}

impl AoType {
    pub fn equipment(&self) -> HashSet<AoEquipment> {
        match self {
            AoType::Bootcamp => HashSet::from([AoEquipment::Coupons]),
            AoType::Heavy => HashSet::from([AoEquipment::Ruck, AoEquipment::Sandbag]),
            AoType::Running => HashSet::from([AoEquipment::RunningShoes]),
            AoType::Rucking => HashSet::from([AoEquipment::Ruck]),
        }
    }
}

mod ao_times {
    use chrono::NaiveTime;

    pub fn five_fifteen() -> NaiveTime {
        NaiveTime::from_hms(5, 15, 0)
    }

    pub fn six() -> NaiveTime {
        NaiveTime::from_hms(6, 0, 0)
    }

    pub fn five() -> NaiveTime {
        NaiveTime::from_hms(5, 0, 0)
    }

    pub fn five_thirty() -> NaiveTime {
        NaiveTime::from_hms(5, 30, 0)
    }
}

impl AO {
    pub fn week_days(&self) -> AoDays {
        match self {
            AO::Bleach => HashSet::from([Weekday::Mon, Weekday::Wed, Weekday::Sat]),
            AO::Gem => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::OldGlory => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::Rebel => HashSet::from([Weekday::Tue, Weekday::Thu]),
            AO::IronMountain => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::RuckershipWest => HashSet::from([Weekday::Fri]),
            AO::RuckershipEast => HashSet::from([Weekday::Fri]),
            AO::Backyard => HashSet::from([Weekday::Wed, Weekday::Fri]),
            AO::Rise => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::WarHorse => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::Bellagio => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::Discovery => HashSet::from([Weekday::Sat]),
            AO::BlackDiamond => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::BlackOps => HashSet::from([Weekday::Sun]),
            AO::DR | AO::Unknown(_) => HashSet::new(),
        }
    }

    pub fn default_time(&self, date: &NaiveDate) -> Option<NaiveTime> {
        let week_day = date.weekday();
        let five = ao_times::five();
        let five_fifteen = ao_times::five_fifteen();
        let five_thirty = ao_times::five_thirty();
        let six = ao_times::six();

        match week_day {
            Weekday::Mon => match self {
                AO::OldGlory => Some(six),
                AO::BlackDiamond => Some(five),
                ao if ao.week_days().contains(&week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Tue => match self {
                AO::IronMountain => Some(five_thirty),
                ao if ao.week_days().contains(&week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Wed => match self {
                AO::OldGlory => Some(six),
                AO::BlackDiamond => Some(five),
                ao if ao.week_days().contains(&week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Thu => match self {
                AO::IronMountain => Some(five_thirty),
                ao if ao.week_days().contains(&week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Fri => match self {
                AO::RuckershipWest | AO::RuckershipEast => Some(five_thirty),
                ao if ao.week_days().contains(&week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Sat => match self {
                ao if ao.week_days().contains(&week_day) => Some(six),
                _ => None,
            },
            Weekday::Sun => match self {
                AO::BlackOps => Some(six),
                _ => None,
            },
        }
    }

    pub fn ao_type(&self) -> AoType {
        match self {
            AO::Bleach => AoType::Heavy,
            AO::BlackDiamond => AoType::Heavy,
            AO::Bellagio => AoType::Bootcamp,
            AO::Backyard => AoType::Bootcamp,
            AO::OldGlory => AoType::Bootcamp,
            AO::Rebel => AoType::Running,
            AO::Discovery => AoType::Bootcamp,
            AO::RuckershipEast | AO::RuckershipWest => AoType::Rucking,
            AO::Rise => AoType::Bootcamp,
            AO::WarHorse => AoType::Bootcamp,
            AO::Gem => AoType::Bootcamp,
            AO::IronMountain => AoType::Bootcamp,
            AO::BlackOps => AoType::Bootcamp,
            AO::DR => AoType::Bootcamp,
            AO::Unknown(_) => AoType::Bootcamp,
        }
    }

    /// whether or not AO is otb.
    pub fn is_otb(&self) -> bool {
        matches!(self, AO::Unknown(_) | AO::DR)
    }

    pub fn channel_id(&self) -> &str {
        match self {
            AO::Bleach => const_names::BLEACH_CHANNEL_ID,
            AO::Gem => const_names::GEM_CHANNEL_ID,
            AO::OldGlory => const_names::OLD_GLORY_CHANNEL_ID,
            AO::Rebel => const_names::REBEL_CHANNEL_ID,
            AO::IronMountain => const_names::IRON_MOUNTAIN_CHANNEL_ID,
            AO::RuckershipWest => const_names::RUCKERSHIP_WEST_CHANNEL_ID,
            AO::RuckershipEast => const_names::RUCKERSHIP_EAST_CHANNEL_ID,
            AO::Backyard => const_names::BACKYARD_CHANNEL_ID,
            AO::Rise => const_names::RISE_CHANNEL_ID,
            AO::WarHorse => const_names::WAR_HORSE_CHANNEL_ID,
            AO::Bellagio => const_names::BELLAGIO_CHANNEL_ID,
            AO::Discovery => const_names::DISCOVERY_CHANNEL_ID,
            AO::BlackDiamond => const_names::BLACK_DIAMOND_CHANNEL_ID,
            AO::BlackOps => const_names::BLACK_OPS_CHANNEL_ID,
            AO::DR => const_names::DR_CHANNEL_ID,
            AO::Unknown(_) => "UNKNOWN",
        }
    }

    /// get google maps link for ao
    pub fn google_maps_link(&self) -> &str {
        match self {
            AO::Bleach => const_names::BLEACH_GOOGLE_MAPS,
            AO::Gem => const_names::GEM_GOOGLE_MAPS,
            AO::OldGlory => const_names::OLD_GLORY_GOOGLE_MAPS,
            AO::Rebel => const_names::REBEL_GOOGLE_MAPS,
            AO::IronMountain => const_names::IRON_MOUNTAIN_GOOGLE_MAPS,
            AO::RuckershipWest | AO::RuckershipEast => "Location Varies",
            AO::Backyard => const_names::BACKYARD_GOOGLE_MAPS,
            AO::Rise => const_names::RISE_GOOGLE_MAPS,
            AO::WarHorse => const_names::WAR_HORSE_GOOGLE_MAPS,
            AO::Bellagio => const_names::BELLAGIO_GOOGLE_MAPS,
            AO::Discovery => const_names::DISCOVERY_GOOGLE_MAPS,
            AO::BlackDiamond => const_names::BLACK_DIAMOND_GOOGLE_MAPS,
            AO::DR | AO::BlackOps => "Location Varies",
            AO::Unknown(_) => "Unknown",
        }
    }

    pub fn from_channel_id(channel_id: &str) -> Self {
        match channel_id {
            const_names::BLEACH_CHANNEL_ID => AO::Bleach,
            const_names::GEM_CHANNEL_ID => AO::Gem,
            const_names::OLD_GLORY_CHANNEL_ID => AO::OldGlory,
            const_names::REBEL_CHANNEL_ID => AO::Rebel,
            const_names::IRON_MOUNTAIN_CHANNEL_ID => AO::IronMountain,
            const_names::RUCKERSHIP_WEST_CHANNEL_ID => AO::RuckershipWest,
            const_names::RUCKERSHIP_EAST_CHANNEL_ID => AO::RuckershipEast,
            const_names::BACKYARD_CHANNEL_ID => AO::Backyard,
            const_names::RISE_CHANNEL_ID => AO::Rise,
            const_names::WAR_HORSE_CHANNEL_ID => AO::WarHorse,
            const_names::BELLAGIO_CHANNEL_ID => AO::Bellagio,
            const_names::DISCOVERY_CHANNEL_ID => AO::Discovery,
            const_names::BLACK_DIAMOND_CHANNEL_ID => AO::BlackDiamond,
            const_names::BLACK_OPS_CHANNEL_ID => AO::BlackOps,
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
            AO::RuckershipWest => AO::RuckershipWest,
            AO::RuckershipEast => AO::RuckershipEast,
            AO::Backyard => AO::Backyard,
            AO::Rise => AO::Rise,
            AO::DR => AO::DR,
            AO::WarHorse => AO::WarHorse,
            AO::Bellagio => AO::Bellagio,
            AO::Discovery => AO::Discovery,
            AO::BlackDiamond => AO::BlackDiamond,
            AO::BlackOps => AO::BlackOps,
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
            AO::RuckershipWest => const_names::RUCKERSHIP_WEST,
            AO::RuckershipEast => const_names::RUCKERSHIP_EAST,
            AO::Backyard => const_names::BACKYARD,
            AO::Rise => const_names::RISE,
            AO::WarHorse => const_names::WAR_HORSE,
            AO::Bellagio => const_names::BELLAGIO,
            AO::Discovery => const_names::DISCOVERY,
            AO::BlackDiamond => const_names::BLACK_DIAMOND,
            AO::BlackOps => const_names::BLACK_OPS,
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
            const_names::RUCKERSHIP_WEST | "rucker-ship" => AO::RuckershipWest,
            const_names::RUCKERSHIP_EAST => AO::RuckershipEast,
            const_names::RISE | "bowler-park" => AO::Rise,
            const_names::WAR_HORSE | "lakeview_park" => AO::WarHorse,
            const_names::BELLAGIO | "bellagio-resort" => AO::Bellagio,
            const_names::BACKYARD => AO::Backyard,
            const_names::DISCOVERY => AO::Discovery,
            const_names::BLACK_DIAMOND => AO::BlackDiamond,
            const_names::BLACK_OPS => AO::BlackOps,
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
        PublicChannels::RuckershipWest => AO::RuckershipWest,
        PublicChannels::RuckershipEast => AO::RuckershipEast,
        PublicChannels::Backyard => AO::Backyard,
        PublicChannels::IronMountain => AO::IronMountain,
        PublicChannels::Bleach => AO::Bleach,
        PublicChannels::Gem => AO::Gem,
        PublicChannels::OldGlory => AO::OldGlory,
        PublicChannels::Rise => AO::Rise,
        PublicChannels::WarHorse => AO::WarHorse,
        PublicChannels::Bellagio => AO::Bellagio,
        PublicChannels::Discovery => AO::Discovery,
        PublicChannels::BlackDiamond => AO::BlackDiamond,
        PublicChannels::BlackOps => AO::BlackOps,
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
    pub const BLEACH_GOOGLE_MAPS: &str = "https://goo.gl/maps/G7u9tB36R3w6bdwr9";
    pub const GEM: &str = "gem";
    pub const GEM_CHANNEL_ID: &str = "C03UBFXVBGD";
    pub const GEM_GOOGLE_MAPS: &str = "https://goo.gl/maps/3UgKoKid9BhPPfyP7";
    pub const OLD_GLORY: &str = "old-glory";
    pub const OLD_GLORY_CHANNEL_ID: &str = "C03TZTPUFRV";
    pub const OLD_GLORY_GOOGLE_MAPS: &str = "https://goo.gl/maps/9CqybhwKbRFKRpst8";
    pub const REBEL: &str = "rebel";
    pub const REBEL_CHANNEL_ID: &str = "C03V463RFRN";
    pub const REBEL_GOOGLE_MAPS: &str = "https://goo.gl/maps/ndw7v3WpFZqfijfZ7";
    pub const IRON_MOUNTAIN: &str = "iron-mountain";
    pub const IRON_MOUNTAIN_CHANNEL_ID: &str = "C03TZTTHDPZ";
    pub const IRON_MOUNTAIN_GOOGLE_MAPS: &str = "https://goo.gl/maps/V3ubQNeSkm8KhGx46";
    pub const RUCKERSHIP_WEST: &str = "ruckership-west";
    pub const RUCKERSHIP_WEST_CHANNEL_ID: &str = "C03V46DGXMW";
    pub const RUCKERSHIP_EAST: &str = "ruckership-east";
    pub const RUCKERSHIP_EAST_CHANNEL_ID: &str = "C04EQQZSFQA";
    pub const BACKYARD: &str = "backyard";
    pub const BACKYARD_CHANNEL_ID: &str = "C03UEBT1QRZ";
    pub const BACKYARD_GOOGLE_MAPS: &str = "https://goo.gl/maps/i7DDdNY6jspdJaBa9";
    pub const RISE: &str = "rise";
    pub const RISE_CHANNEL_ID: &str = "C03UT46303T";
    pub const RISE_GOOGLE_MAPS: &str = "https://goo.gl/maps/wqZ1UD8DEAUCJTku9";
    pub const WAR_HORSE: &str = "warhorse";
    pub const WAR_HORSE_CHANNEL_ID: &str = "C0425DL9MT7";
    pub const WAR_HORSE_GOOGLE_MAPS: &str = "https://goo.gl/maps/oariasYawYa5o7zs9";
    pub const BELLAGIO: &str = "bellagio";
    pub const BELLAGIO_CHANNEL_ID: &str = "C045SMRL43X";
    pub const BELLAGIO_GOOGLE_MAPS: &str = "https://goo.gl/maps/a7EcVdttBEi1kiQx7";
    pub const DISCOVERY: &str = "discovery-park";
    pub const DISCOVERY_CHANNEL_ID: &str = "C04B2DX8CCW";
    pub const DISCOVERY_GOOGLE_MAPS: &str = "https://goo.gl/maps/zJkeWpgpS93MqhEU7";
    pub const DR: &str = "dr";
    pub const DR_CHANNEL_ID: &str = "C03U7U9T7HU";
    pub const BLACK_DIAMOND: &str = "black-diamond";
    pub const BLACK_DIAMOND_CHANNEL_ID: &str = "C04QQF5M8GL";
    pub const BLACK_DIAMOND_GOOGLE_MAPS: &str = "https://goo.gl/maps/a7EcVdttBEi1kiQx7";
    pub const BLACK_OPS: &str = "black-ops";
    pub const BLACK_OPS_CHANNEL_ID: &str = "C050HTBNU3B";

    /// full list of active aos
    pub const AO_LIST: [AO; 14] = [
        AO::Bleach,
        AO::Gem,
        AO::OldGlory,
        AO::Rebel,
        AO::IronMountain,
        AO::RuckershipWest,
        AO::RuckershipEast,
        AO::Backyard,
        AO::Rise,
        AO::WarHorse,
        AO::Bellagio,
        AO::Discovery,
        AO::BlackDiamond,
        AO::BlackOps,
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

    #[test]
    fn bellagio() {
        let ao = AO::from("ao-bellagio-resort".to_string());
        assert_eq!(ao, AO::Bellagio);
        let ao = AO::from("ao-bellagio".to_string());
        assert_eq!(ao, AO::Bellagio);
    }
}
