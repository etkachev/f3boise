use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::db::queries::pre_blasts::PreBlastJsonFullData;
use crate::shared::string_utils::string_vec_to_hash;
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post::PreBlastPost;
use chrono::{NaiveDate, NaiveTime};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct PreBlastData {
    /// possible id of preblast if saved in db
    pub id: Option<String>,
    /// AO this preblast is part of
    pub ao: AO,
    /// title of preblast
    pub title: String,
    /// list of Q's that led
    pub qs: HashSet<String>,
    /// date that workout happened
    pub date: NaiveDate,
    /// scheduled start time of workout
    pub start_time: NaiveTime,
    /// the why of preblast
    pub why: String,
    /// equipment needed for preblast
    pub equipment: HashSet<AoEquipment>,
    pub fng_message: Option<String>,
    pub mole_skin: Option<String>,
    pub img_ids: HashSet<String>,
}

impl PreBlastData {
    pub fn with_qs(mut self, qs: &HashSet<String>, users: HashMap<String, String>) -> Self {
        let qs = qs.iter().fold(HashSet::<String>::new(), |mut acc, q| {
            if let Some(name) = users.get(q.as_str()) {
                acc.insert(name.to_string());
            } else {
                acc.insert(q.to_string());
            }
            acc
        });
        self.qs = qs;
        self
    }

    pub fn equipment_option_elements(&self) -> Vec<OptionElement> {
        self.equipment.iter().map(OptionElement::from).collect()
    }

    pub fn official_equipment_option_elements(&self) -> Vec<OptionElement> {
        self.equipment
            .iter()
            .filter_map(|item| match item {
                AoEquipment::Other(_) => None,
                rest => Some(OptionElement::from(rest)),
            })
            .collect()
    }

    pub fn other_equipment_option_elements(&self) -> String {
        self.equipment
            .iter()
            .filter_map(|item| match item {
                AoEquipment::Other(other) => Some(other.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl From<&PreBlastPost> for PreBlastData {
    fn from(value: &PreBlastPost) -> Self {
        PreBlastData {
            id: None,
            ao: value.ao.clone(),
            title: value.title.to_string(),
            qs: HashSet::new(),
            date: value.date,
            start_time: value.start_time,
            why: value.why.to_string(),
            equipment: value.equipment.clone(),
            fng_message: value.fng_message.clone(),
            mole_skin: value.mole_skin.clone(),
            img_ids: value.img_ids.clone(),
        }
    }
}

impl From<PreBlastJsonFullData> for PreBlastData {
    fn from(value: PreBlastJsonFullData) -> Self {
        let qs = string_vec_to_hash(&value.qs);
        let equipment = string_vec_to_hash(&value.equipment.unwrap_or_default());
        let equipment = equipment.iter().fold(HashSet::new(), |mut acc, item| {
            if let Ok(item) = AoEquipment::from_str(item) {
                acc.insert(item);
            } else {
                acc.insert(AoEquipment::Other(item.to_string()));
            }
            acc
        });
        let img_ids = string_vec_to_hash(&value.img_ids.unwrap_or_default());

        PreBlastData {
            id: Some(value.id.to_string()),
            qs,
            ao: AO::from_channel_id(value.channel_id.as_str()),
            title: value.title.to_string(),
            date: value.date,
            start_time: value.start_time,
            why: value.why.to_string(),
            equipment,
            fng_message: value.fng_message.clone(),
            mole_skin: value.mole_skin.clone(),
            img_ids,
        }
    }
}
