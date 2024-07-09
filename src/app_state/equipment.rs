use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::block_elements::OptionElement;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AoEquipment {
    Coupons,
    Sandbag,
    Ruck,
    WeightVest,
    RunningShoes,
    Headlamp,
    HeartRateMonitor,
    StopWatch,
    Other(String),
}

impl From<AoEquipment> for OptionElement {
    fn from(value: AoEquipment) -> Self {
        OptionElement::from(&value)
    }
}

impl From<&AoEquipment> for OptionElement {
    fn from(value: &AoEquipment) -> Self {
        match value {
            AoEquipment::Coupons => {
                OptionElement::new(&format!("{} 🧱", AoEquipment::Coupons), "coupon")
            }
            AoEquipment::Sandbag => {
                OptionElement::new(&format!("{} 👝", AoEquipment::Sandbag), "sandbag")
            }
            AoEquipment::Ruck => OptionElement::new(&format!("{} 🎒", AoEquipment::Ruck), "ruck"),
            AoEquipment::RunningShoes => {
                OptionElement::new(&format!("{} 👟", AoEquipment::RunningShoes), "shoes")
            }
            AoEquipment::Headlamp => {
                OptionElement::new(&format!("{} 🔦", AoEquipment::Headlamp), "headlamp")
            }
            AoEquipment::HeartRateMonitor => OptionElement::new(
                &format!("{} 🫀", AoEquipment::HeartRateMonitor),
                "hr_monitor",
            ),
            AoEquipment::WeightVest => {
                OptionElement::new(&format!("{} 🦺", AoEquipment::WeightVest), "weight_vest")
            }
            AoEquipment::StopWatch => {
                OptionElement::new(&format!("{} ⏱️", AoEquipment::StopWatch), "stop_watch")
            }
            AoEquipment::Other(other) => OptionElement::new(other, other),
        }
    }
}

impl FromStr for AoEquipment {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let result = match text.to_lowercase().as_str() {
            "coupon" | "coupons" => AoEquipment::Coupons,
            "sandbag" | "sandbags" | "sb" => AoEquipment::Sandbag,
            "ruck" | "rucksack" => AoEquipment::Ruck,
            "running_shoes" | "shoes" | "running shoes" => AoEquipment::RunningShoes,
            "weightvest" | "weight_vest" | "weight vest" | "weighted vest" => {
                AoEquipment::WeightVest
            }
            "hr monitor" | "hr_monitor" | "heart rate monitor" | "heart_rate_monitor" => {
                AoEquipment::HeartRateMonitor
            }
            "headlamp" | "head lamp" | "head_lamp" => AoEquipment::Headlamp,
            "stopwatch" | "stop_watch" => AoEquipment::StopWatch,
            other => AoEquipment::Other(other.to_string()),
        };

        Ok(result)
    }
}

impl Display for AoEquipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AoEquipment::Coupons => String::from("Coupons"),
            AoEquipment::Sandbag => String::from("Sandbag"),
            AoEquipment::Ruck => String::from("Ruck"),
            AoEquipment::WeightVest => String::from("Weight Vest"),
            AoEquipment::RunningShoes => String::from("Running Shoes"),
            AoEquipment::Headlamp => String::from("Headlamp"),
            AoEquipment::HeartRateMonitor => String::from("HR Monitor"),
            AoEquipment::StopWatch => String::from("Stopwatch"),
            AoEquipment::Other(other) => other.to_string(),
        };
        write!(f, "{}", str)
    }
}
