use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::block_elements::OptionElement;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AoEquipment {
    Coupons,
    Sandbag,
    Ruck,
    RunningShoes,
    Headlamp,
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
            AoEquipment::Coupons => OptionElement::new(
                &format!("{} 🧱", AoEquipment::Coupons.to_string()),
                "coupon",
            ),
            AoEquipment::Sandbag => OptionElement::new(
                &format!("{} 👝", AoEquipment::Sandbag.to_string()),
                "sandbag",
            ),
            AoEquipment::Ruck => {
                OptionElement::new(&format!("{} 🎒", AoEquipment::Ruck.to_string()), "ruck")
            }
            AoEquipment::RunningShoes => OptionElement::new(
                &format!("{} 👟", AoEquipment::RunningShoes.to_string()),
                "shoes",
            ),
            AoEquipment::Headlamp => OptionElement::new(
                &format!("{} 🔦", AoEquipment::Headlamp.to_string()),
                "headlamp",
            ),
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
            "headlamp" | "head lamp" | "head_lamp" => AoEquipment::Headlamp,
            other => AoEquipment::Other(other.to_string()),
        };

        Ok(result)
    }
}

impl ToString for AoEquipment {
    fn to_string(&self) -> String {
        match self {
            AoEquipment::Coupons => String::from("Coupons"),
            AoEquipment::Sandbag => String::from("Sandbag"),
            AoEquipment::Ruck => String::from("Ruck"),
            AoEquipment::RunningShoes => String::from("Running Shoes"),
            AoEquipment::Headlamp => String::from("Headlamp"),
            AoEquipment::Other(other) => other.to_string(),
        }
    }
}
