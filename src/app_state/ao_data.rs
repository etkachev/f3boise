use crate::app_state::equipment::AoEquipment;
use crate::slack_api::channels::public_channels::PublicChannels;
use chrono::{Duration, NaiveTime, Weekday};
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
    Tower,
    BlackDiamond,
    BlackOps,
    CamelsBack,
    ReidMerrill,
    GooseDynasty,
    RetaHuskey,
    FirstF,
    DR,
    Unknown(String),
}

/// days of the week the ao is open
pub type AoDays = HashSet<Weekday>;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum AoType {
    Bootcamp,
    Heavy,
    HighIntensity,
    Running,
    Rucking,
}

impl ToString for AoType {
    fn to_string(&self) -> String {
        match self {
            AoType::Bootcamp => String::from("Bootcamp"),
            AoType::HighIntensity => String::from("High Intensity"),
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
            AoType::HighIntensity => HashSet::from([AoEquipment::Ruck, AoEquipment::Sandbag]),
            AoType::Heavy => HashSet::from([AoEquipment::Ruck, AoEquipment::Sandbag]),
            AoType::Running => HashSet::from([AoEquipment::RunningShoes]),
            AoType::Rucking => HashSet::from([AoEquipment::Ruck, AoEquipment::Headlamp]),
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
}

impl AO {
    pub fn week_days(&self) -> AoDays {
        match self {
            AO::Bleach => HashSet::from([Weekday::Mon, Weekday::Wed, Weekday::Sat]),
            AO::Gem => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::OldGlory => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::Rebel => HashSet::from([Weekday::Mon, Weekday::Tue, Weekday::Thu]),
            AO::IronMountain => HashSet::from([Weekday::Tue, Weekday::Sat]),
            AO::RuckershipWest => HashSet::from([Weekday::Fri]),
            AO::RuckershipEast => HashSet::from([Weekday::Sat]),
            AO::Backyard => HashSet::from([Weekday::Wed, Weekday::Fri]),
            AO::Rise => HashSet::from([Weekday::Mon, Weekday::Wed]),
            AO::WarHorse => HashSet::from([Weekday::Mon, Weekday::Thu]),
            AO::Bellagio => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::Tower => HashSet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat]),
            AO::BlackDiamond => HashSet::from([Weekday::Wed]),
            AO::BlackOps => HashSet::new(),
            AO::FirstF => HashSet::new(),
            AO::CamelsBack => HashSet::from([Weekday::Wed, Weekday::Fri]),
            AO::ReidMerrill => HashSet::from([Weekday::Mon, Weekday::Fri]),
            AO::GooseDynasty => HashSet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]),
            AO::RetaHuskey => HashSet::from([Weekday::Sat]),
            AO::DR | AO::Unknown(_) => HashSet::new(),
        }
    }

    pub fn friendly_name(&self) -> &str {
        match self {
            AO::Bleach => "Bleach",
            AO::Gem => "Gem",
            AO::OldGlory => "Old Glory",
            AO::Rebel => "Rebel",
            AO::IronMountain => "Iron Mountain",
            AO::RuckershipWest => "Ruckership West",
            AO::RuckershipEast => "Ruckership East",
            AO::Backyard => "Backyard",
            AO::Rise => "Rise",
            AO::WarHorse => "Warhorse",
            AO::Bellagio => "Bellagio",
            AO::Tower => "Tower",
            AO::BlackDiamond => "Black Diamond",
            AO::BlackOps => "Black Ops",
            AO::FirstF => "1st F",
            AO::CamelsBack => "Camel's Back",
            AO::ReidMerrill => "Reid Merrill",
            AO::GooseDynasty => "Goose Dynasty",
            AO::RetaHuskey => "Reta Huskey Park",
            AO::DR => "DR",
            AO::Unknown(_) => "UNKNOWN",
        }
    }

    pub fn workout_length(&self, week_day: &Weekday) -> Option<i64> {
        match week_day {
            Weekday::Mon => match self {
                AO::BlackDiamond => Some(60),
                _ => Some(45),
            },
            Weekday::Tue => Some(45),
            Weekday::Wed => match self {
                AO::BlackDiamond => Some(60),
                _ => Some(45),
            },
            Weekday::Thu => Some(45),
            Weekday::Fri => match self {
                AO::Backyard => Some(45),
                AO::RuckershipWest | AO::RuckershipEast => Some(60),
                _ => Some(45),
            },
            Weekday::Sat => Some(60),
            Weekday::Sun => None,
        }
    }

    pub fn start_end_times(&self, week_day: &Weekday) -> Option<(NaiveTime, NaiveTime)> {
        match (self.default_time(week_day), self.workout_length(week_day)) {
            (Some(start), Some(minutes)) => {
                let end_time = start + Duration::minutes(minutes);
                Some((start, end_time))
            }
            _ => None,
        }
    }

    pub fn default_time(&self, week_day: &Weekday) -> Option<NaiveTime> {
        let five = ao_times::five();
        let five_fifteen = ao_times::five_fifteen();
        let six = ao_times::six();

        match week_day {
            Weekday::Mon => match self {
                AO::OldGlory => Some(six),
                AO::BlackDiamond => Some(five),
                ao if ao.week_days().contains(week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Tue => match self {
                ao if ao.week_days().contains(week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Wed => match self {
                AO::OldGlory => Some(six),
                AO::BlackDiamond => Some(five),
                ao if ao.week_days().contains(week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Thu => match self {
                ao if ao.week_days().contains(week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Fri => match self {
                AO::RuckershipWest => Some(five_fifteen),
                ao if ao.week_days().contains(week_day) => Some(five_fifteen),
                _ => None,
            },
            Weekday::Sat => match self {
                ao if ao.week_days().contains(week_day) => Some(six),
                _ => None,
            },
            Weekday::Sun => None,
        }
    }

    pub fn ao_type(&self) -> AoType {
        match self {
            AO::Bleach => AoType::Heavy,
            AO::BlackDiamond => AoType::HighIntensity,
            AO::Bellagio => AoType::Bootcamp,
            AO::Backyard => AoType::Bootcamp,
            AO::OldGlory => AoType::Bootcamp,
            AO::Rebel => AoType::Running,
            AO::Tower => AoType::Bootcamp,
            AO::RuckershipEast | AO::RuckershipWest => AoType::Rucking,
            AO::Rise => AoType::Bootcamp,
            AO::WarHorse => AoType::Bootcamp,
            AO::Gem => AoType::Bootcamp,
            AO::IronMountain => AoType::Bootcamp,
            AO::BlackOps => AoType::Bootcamp,
            AO::FirstF => AoType::Bootcamp,
            AO::CamelsBack => AoType::Bootcamp,
            AO::ReidMerrill => AoType::Bootcamp,
            AO::GooseDynasty => AoType::Bootcamp,
            AO::RetaHuskey => AoType::Running,
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
            AO::Tower => const_names::THE_TOWER_CHANNEL_ID,
            AO::BlackDiamond => const_names::BLACK_DIAMOND_CHANNEL_ID,
            AO::BlackOps => const_names::BLACK_OPS_CHANNEL_ID,
            AO::FirstF => const_names::FIRST_F_CHANNEL_ID,
            AO::CamelsBack => const_names::CAMELS_BACK_CHANNEL_ID,
            AO::ReidMerrill => const_names::REID_MERRILL_CHANNEL_ID,
            AO::GooseDynasty => const_names::GOOSE_DYNASTY_CHANNEL_ID,
            AO::RetaHuskey => const_names::RETA_HUSKEY_CHANNEL_ID,
            AO::DR => const_names::DR_CHANNEL_ID,
            AO::Unknown(_) => "UNKNOWN",
        }
    }

    pub fn address(&self) -> Option<&str> {
        match self {
            AO::Backyard => Some("2400 S Stoddard Rd, Meridian, ID 83642"),
            AO::BlackDiamond => Some("Kleiner Park Loop, Meridian, ID 83642"),
            AO::Bleach => Some("801 Aurora Dr, Boise, ID 83709"),
            AO::OldGlory => Some("3064 W Malta Dr, Meridian, ID 83646"),
            AO::Rise => Some("4403 S Surprise Way, Boise, ID 83716"),
            AO::WarHorse => Some("7999 Cherry Ln, Nampa, ID 83687"),
            AO::Bellagio => Some("Kleiner Park Loop, Meridian, ID 83642"),
            AO::Gem => Some("3423 N Meridian Rd, Meridian, ID 83642"),
            AO::IronMountain => Some("75 Marjorie Ave, Middleton, ID 83644"),
            AO::Rebel => Some("3801 E Hill Park Street, Meridian, ID 83642"),
            AO::Tower => Some("2121 E Lake Hazel Rd, Meridian, ID 83642"),
            AO::CamelsBack => Some("1200 Heron St, Boise, ID 83702"),
            AO::ReidMerrill => Some("637 E Shore Dr, Eagle, ID 83616"),
            AO::GooseDynasty => Some("2815 S Maple Grove Rd, Boise, ID 83709"),
            AO::RetaHuskey => Some("2887 W Tubac Dr, Meridian, ID  83646"),
            AO::RuckershipEast
            | AO::RuckershipWest
            | AO::BlackOps
            | AO::FirstF
            | AO::DR
            | AO::Unknown(_) => None,
        }
    }

    // optionally return google maps link
    pub fn real_map_url(&self) -> Option<&str> {
        match self {
            AO::Bleach => Some(const_names::BLEACH_GOOGLE_MAPS),
            AO::Gem => Some(const_names::GEM_GOOGLE_MAPS),
            AO::OldGlory => Some(const_names::OLD_GLORY_GOOGLE_MAPS),
            AO::Rebel => Some(const_names::REBEL_GOOGLE_MAPS),
            AO::IronMountain => Some(const_names::IRON_MOUNTAIN_GOOGLE_MAPS),
            AO::Backyard => Some(const_names::BACKYARD_GOOGLE_MAPS),
            AO::Rise => Some(const_names::RISE_GOOGLE_MAPS),
            AO::WarHorse => Some(const_names::WAR_HORSE_GOOGLE_MAPS),
            AO::Bellagio => Some(const_names::BELLAGIO_GOOGLE_MAPS),
            AO::Tower => Some(const_names::THE_TOWER_GOOGLE_MAPS),
            AO::BlackDiamond => Some(const_names::BLACK_DIAMOND_GOOGLE_MAPS),
            AO::CamelsBack => Some(const_names::CAMELS_BACK_GOOGLE_MAPS),
            AO::ReidMerrill => Some(const_names::REID_MERRILL_GOOGLE_MAPS),
            AO::GooseDynasty => Some(const_names::GOOSE_DYNASTY_GOOGLE_MAPS),
            AO::RetaHuskey => Some(const_names::RETA_HUSKEY_GOOGLE_MAPS),
            AO::RuckershipWest | AO::RuckershipEast => None,
            AO::DR | AO::BlackOps | AO::FirstF => None,
            AO::Unknown(_) => None,
        }
    }

    /// get google maps link for ao (returns generic text if not available)
    pub fn google_maps_link(&self) -> &str {
        self.real_map_url().unwrap_or(match self {
            AO::RuckershipWest | AO::RuckershipEast => "Location Varies",
            AO::DR | AO::BlackOps | AO::FirstF => "Location Varies",
            AO::Unknown(_) => "Unknown",
            _ => "",
        })
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
            const_names::THE_TOWER_CHANNEL_ID => AO::Tower,
            const_names::BLACK_DIAMOND_CHANNEL_ID => AO::BlackDiamond,
            const_names::BLACK_OPS_CHANNEL_ID => AO::BlackOps,
            const_names::FIRST_F_CHANNEL_ID => AO::FirstF,
            const_names::CAMELS_BACK_CHANNEL_ID => AO::CamelsBack,
            const_names::REID_MERRILL_CHANNEL_ID => AO::ReidMerrill,
            const_names::GOOSE_DYNASTY_CHANNEL_ID => AO::GooseDynasty,
            const_names::RETA_HUSKEY_CHANNEL_ID => AO::RetaHuskey,
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
            AO::Tower => AO::Tower,
            AO::BlackDiamond => AO::BlackDiamond,
            AO::BlackOps => AO::BlackOps,
            AO::FirstF => AO::FirstF,
            AO::CamelsBack => AO::CamelsBack,
            AO::ReidMerrill => AO::ReidMerrill,
            AO::GooseDynasty => AO::GooseDynasty,
            AO::RetaHuskey => AO::RetaHuskey,
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
            AO::Tower => const_names::THE_TOWER,
            AO::BlackDiamond => const_names::BLACK_DIAMOND,
            AO::BlackOps => const_names::BLACK_OPS,
            AO::FirstF => const_names::FIRST_F,
            AO::CamelsBack => const_names::CAMELS_BACK,
            AO::ReidMerrill => const_names::REID_MERRILL,
            AO::GooseDynasty => const_names::GOOSE_DYNASTY,
            AO::RetaHuskey => const_names::RETA_HUSKEY,
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
            const_names::THE_TOWER | "the-tower" | "discovery-park" => AO::Tower,
            const_names::BLACK_DIAMOND => AO::BlackDiamond,
            const_names::BLACK_OPS => AO::BlackOps,
            const_names::FIRST_F => AO::FirstF,
            const_names::CAMELS_BACK => AO::CamelsBack,
            const_names::REID_MERRILL | "otb-reid-merrill-park" => AO::ReidMerrill,
            const_names::GOOSE_DYNASTY | "otb-goose-dynasty" => AO::GooseDynasty,
            const_names::RETA_HUSKEY => AO::RetaHuskey,
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
        PublicChannels::Tower => AO::Tower,
        PublicChannels::BlackDiamond => AO::BlackDiamond,
        PublicChannels::BlackOps => AO::BlackOps,
        PublicChannels::FirstF => AO::FirstF,
        PublicChannels::CamelsBack => AO::CamelsBack,
        PublicChannels::ReidMerrill => AO::ReidMerrill,
        PublicChannels::GooseDynasty => AO::GooseDynasty,
        PublicChannels::RetaHuskey => AO::RetaHuskey,
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
    pub const WAR_HORSE_GOOGLE_MAPS: &str = "https://maps.app.goo.gl/6bcedmbincGxMtmM9";
    pub const BELLAGIO: &str = "bellagio";
    pub const BELLAGIO_CHANNEL_ID: &str = "C045SMRL43X";
    pub const BELLAGIO_GOOGLE_MAPS: &str = "https://goo.gl/maps/5xFSnaT57Ws1JAZR6";
    pub const THE_TOWER: &str = "tower";
    pub const THE_TOWER_CHANNEL_ID: &str = "C04B2DX8CCW";
    pub const THE_TOWER_GOOGLE_MAPS: &str = "https://goo.gl/maps/zJkeWpgpS93MqhEU7";
    pub const DR: &str = "dr";
    pub const DR_CHANNEL_ID: &str = "C03U7U9T7HU";
    pub const BLACK_DIAMOND: &str = "black-diamond";
    pub const BLACK_DIAMOND_CHANNEL_ID: &str = "C04QQF5M8GL";
    pub const BLACK_DIAMOND_GOOGLE_MAPS: &str = "https://goo.gl/maps/hsSKRSVHjifx4HdV6";
    pub const BLACK_OPS: &str = "black-ops";
    pub const BLACK_OPS_CHANNEL_ID: &str = "C050HTBNU3B";
    pub const FIRST_F: &str = "1st-f";
    pub const FIRST_F_CHANNEL_ID: &str = "C03V46PFL7J";
    pub const CAMELS_BACK: &str = "camels-back";
    pub const CAMELS_BACK_CHANNEL_ID: &str = "C05AJDFUBM4";
    pub const CAMELS_BACK_GOOGLE_MAPS: &str = "https://maps.app.goo.gl/28RexfPU7yd7z2Zg8";
    pub const REID_MERRILL: &str = "reid-merrill-park";
    pub const REID_MERRILL_CHANNEL_ID: &str = "C05UUDKULGY";
    pub const REID_MERRILL_GOOGLE_MAPS: &str = "https://maps.app.goo.gl/mycHr8xipwwqhhSu6";
    pub const GOOSE_DYNASTY: &str = "goose-dynasty";
    pub const GOOSE_DYNASTY_CHANNEL_ID: &str = "C06DP3D5VTK";
    pub const GOOSE_DYNASTY_GOOGLE_MAPS: &str = "https://maps.app.goo.gl/B6xDeMgoV9LMbuke9";
    pub const RETA_HUSKEY: &str = "otb-reta-huskey-park";
    pub const RETA_HUSKEY_CHANNEL_ID: &str = "C06LMEEDC1F";
    pub const RETA_HUSKEY_GOOGLE_MAPS: &str = "https://maps.app.goo.gl/qm2cW7sqki8q2hgg7";

    /// full list of active aos
    pub const AO_LIST: [AO; 19] = [
        AO::Backyard,
        AO::Bellagio,
        AO::BlackDiamond,
        AO::BlackOps,
        AO::FirstF,
        AO::Bleach,
        AO::CamelsBack,
        AO::Gem,
        AO::IronMountain,
        AO::OldGlory,
        AO::Rebel,
        AO::ReidMerrill,
        AO::RetaHuskey,
        AO::Rise,
        AO::GooseDynasty,
        AO::RuckershipEast,
        AO::RuckershipWest,
        AO::Tower,
        AO::WarHorse,
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

    #[test]
    fn black_ops() {
        let ao = AO::from("black-ops".to_string());
        assert_eq!(ao, AO::BlackOps);
    }

    #[test]
    fn ruckership_start_time() {
        let ao = AO::RuckershipWest;
        let start_time = ao.default_time(&Weekday::Fri).unwrap();
        assert_eq!(start_time, NaiveTime::from_hms(5, 15, 0));
    }

    #[test]
    fn molenaar_park() {
        let ao = AO::from("otb-goose-dynasty".to_string());
        assert_eq!(ao, AO::GooseDynasty);
    }
}
