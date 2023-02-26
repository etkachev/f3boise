use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use chrono::{NaiveDate, NaiveTime};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
pub struct PreBlastPost {
    pub title: String,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub ao: AO,
    pub qs: Vec<String>,
    pub why: String,
    pub equipment: HashSet<AoEquipment>,
    pub fng_message: Option<String>,
    pub mole_skin: String,
    pub post_where: PreBlastWhere,
}

impl PreBlastPost {
    /// parse to slack list of users
    pub fn qs_list(&self) -> String {
        self.qs
            .iter()
            .map(|q| format!("<@{}>", q))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn equipment_list(&self) -> String {
        self.equipment
            .iter()
            .map(|item| OptionElement::from(item).text.text)
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Debug)]
pub enum PreBlastWhere {
    AoChannel,
    CurrentChannel(String),
    Myself,
}

impl Default for PreBlastWhere {
    fn default() -> Self {
        PreBlastWhere::AoChannel
    }
}

impl ToString for PreBlastWhere {
    fn to_string(&self) -> String {
        match self {
            PreBlastWhere::AoChannel => String::from("Ao Channel"),
            PreBlastWhere::CurrentChannel(_) => String::from("Current Channel"),
            PreBlastWhere::Myself => String::from("Me"),
        }
    }
}

impl FromStr for PreBlastWhere {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let split_text = text.split_once("::").unwrap_or((text, ""));
        match split_text {
            ("ao_channel", _) => Ok(PreBlastWhere::AoChannel),
            ("current_channel", id) => Ok(PreBlastWhere::CurrentChannel(id.to_string())),
            ("self", _) => Ok(PreBlastWhere::Myself),
            _ => Err(AppError::General("Could not parse".to_string())),
        }
    }
}

impl From<PreBlastWhere> for OptionElement {
    fn from(value: PreBlastWhere) -> Self {
        match value {
            PreBlastWhere::AoChannel => {
                OptionElement::new(&PreBlastWhere::AoChannel.to_string(), "ao_channel")
            }
            PreBlastWhere::CurrentChannel(channel_id) => OptionElement::new(
                &PreBlastWhere::CurrentChannel(channel_id.to_string()).to_string(),
                &format!("current_channel::{channel_id}"),
            ),
            PreBlastWhere::Myself => OptionElement::new(&PreBlastWhere::Myself.to_string(), "self"),
        }
    }
}

impl From<HashMap<String, BasicValue>> for PreBlastPost {
    fn from(value: HashMap<String, BasicValue>) -> Self {
        let ao = {
            if let Some(BasicValue::Single(ao_value)) = value.get(pre_blast_action_ids::AO_SELECT) {
                AO::from_channel_id(ao_value)
            } else {
                AO::Unknown("Not parsed".to_string())
            }
        };
        let title = {
            if let Some(BasicValue::Single(title)) = value.get(pre_blast_action_ids::TITLE) {
                title.to_string()
            } else {
                String::from("Title")
            }
        };

        let date = {
            if let Some(BasicValue::Single(date)) = value.get(pre_blast_action_ids::DATE) {
                NaiveDate::from_str(date).unwrap_or_default()
            } else {
                NaiveDate::default()
            }
        };

        let start_time = {
            if let Some(BasicValue::Single(time)) = value.get(pre_blast_action_ids::TIME_SELECT) {
                NaiveTime::parse_from_str(time, "%H:%M").unwrap_or_default()
            } else {
                NaiveTime::default()
            }
        };

        let qs = {
            if let Some(BasicValue::Multi(qs)) = value.get(pre_blast_action_ids::QS) {
                qs.iter().map(|q| q.to_string()).collect::<Vec<String>>()
            } else {
                Vec::<String>::new()
            }
        };

        let why = {
            if let Some(BasicValue::Single(text)) = value.get(pre_blast_action_ids::WHY) {
                text.to_string()
            } else {
                String::new()
            }
        };

        let equipment = {
            if let Some(BasicValue::Multi(list)) = value.get(pre_blast_action_ids::EQUIPMENT) {
                list.iter()
                    .fold(HashSet::<AoEquipment>::new(), |mut acc, item| {
                        acc.insert(AoEquipment::from_str(item).unwrap());
                        acc
                    })
            } else {
                HashSet::<AoEquipment>::new()
            }
        };

        let fng_message = {
            if let Some(BasicValue::Single(message)) = value.get(pre_blast_action_ids::FNGS) {
                if !message.is_empty() {
                    Some(message.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        };

        let mole_skin = {
            if let Some(BasicValue::Single(message)) = value.get(pre_blast_action_ids::MOLE_SKINE) {
                message.to_string()
            } else {
                String::new()
            }
        };

        let post_where = {
            if let Some(BasicValue::Single(post_where)) =
                value.get(pre_blast_action_ids::WHERE_POST)
            {
                PreBlastWhere::from_str(post_where).unwrap_or(PreBlastWhere::AoChannel)
            } else {
                PreBlastWhere::default()
            }
        };

        PreBlastPost {
            title,
            date,
            start_time,
            ao,
            qs,
            why,
            equipment,
            fng_message,
            mole_skin,
            post_where,
        }
    }
}

pub mod pre_blast_action_ids {
    pub const AO_SELECT: &str = "ao.select";
    pub const TIME_SELECT: &str = "time.select";
    pub const TITLE: &str = "title.input";
    pub const DATE: &str = "date.select";
    pub const QS: &str = "qs.select";
    pub const WHY: &str = "why.input";
    pub const EQUIPMENT: &str = "equipment.select";
    pub const OTHER_EQUIPMENT: &str = "other_equipment.input";
    pub const FNGS: &str = "fngs.input";
    pub const MOLE_SKINE: &str = "moleskin.textbox";
    pub const WHERE_POST: &str = "where_to_post.select";
}

pub fn convert_to_message(post: PreBlastPost) -> PostMessageRequest {
    let channel_id = match &post.post_where {
        PreBlastWhere::AoChannel => post.ao.channel_id().to_string(),
        PreBlastWhere::CurrentChannel(id) => id.to_string(),
        // TODO
        PreBlastWhere::Myself => "TODO".to_string(),
    };
    let block_builder = BlockBuilder::new()
        .section_markdown(&format!("*Preblast: {}*", post.title))
        .section_markdown(&format!("*Date*: {}", post.date))
        .section_markdown(&format!("*Time*: {}", post.start_time.format("%H:%M")))
        .section_markdown(&format!("*Where*: <#{}>", post.ao.channel_id()))
        .section_markdown(&format!("*Q(s)*: {}", post.qs_list()))
        .divider()
        .section_markdown(&format!("*Why*: {}", post.why))
        .section_markdown(&format!("*Equipment*: {}", post.equipment_list()))
        .section_markdown(&format!("*FNGs*: {}", post.fng_message.unwrap_or_default()))
        .section_markdown(&post.mole_skin)
        .divider();

    PostMessageRequest {
        channel: channel_id,
        blocks: block_builder.blocks,
    }
}
