use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::db::queries::users::get_user_by_slack_id;
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::slash_commands::modal_utils::{value_utils, BlastWhere};
use chrono::{NaiveDate, NaiveTime};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct PreBlastPost {
    pub title: String,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub ao: AO,
    pub qs: HashSet<String>,
    pub why: String,
    pub equipment: HashSet<AoEquipment>,
    pub fng_message: Option<String>,
    pub mole_skin: Option<String>,
    pub post_where: BlastWhere,
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
        if self.equipment.is_empty() {
            return String::from("None");
        }

        self.equipment
            .iter()
            .map(|item| OptionElement::from(item).text.text)
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_first_q(&self) -> Option<String> {
        self.qs
            .iter()
            .map(|q| q.to_string())
            .collect::<Vec<String>>()
            .first()
            .map(|q| q.to_string())
    }
}

impl From<HashMap<String, BasicValue>> for PreBlastPost {
    fn from(value: HashMap<String, BasicValue>) -> Self {
        let ao = value
            .get(pre_blast_action_ids::AO_SELECT)
            .map(value_utils::get_ao_value)
            .unwrap_or_else(|| AO::Unknown("Not Parsed".to_string()));

        let title = value
            .get(pre_blast_action_ids::TITLE)
            .map(value_utils::get_single_string)
            .unwrap_or_else(|| String::from("Title"));

        let date = value
            .get(pre_blast_action_ids::DATE)
            .map(value_utils::get_single_date)
            .unwrap_or_default();

        let start_time = value
            .get(pre_blast_action_ids::TIME_SELECT)
            .map(value_utils::get_single_time)
            .unwrap_or_default();

        let qs = value
            .get(pre_blast_action_ids::QS)
            .map(value_utils::get_hash_set_strings_from_multi)
            .unwrap_or_default();

        let why = value
            .get(pre_blast_action_ids::WHY)
            .map(value_utils::get_single_string)
            .unwrap_or_default();

        let mut equipment = value
            .get(pre_blast_action_ids::EQUIPMENT)
            .map(value_utils::get_equipment_multi_value)
            .unwrap_or_default();

        let other_equipment = value
            .get(pre_blast_action_ids::OTHER_EQUIPMENT)
            .map(|v| {
                v.get_single().unwrap_or_default().split(',').fold(
                    HashSet::<AoEquipment>::new(),
                    |mut acc, item| {
                        let item = item.trim();
                        if !item.is_empty() {
                            acc.insert(AoEquipment::Other(item.to_string()));
                        }
                        acc
                    },
                )
            })
            .unwrap_or_default();

        equipment.extend(other_equipment);

        let fng_message = value
            .get(pre_blast_action_ids::FNGS)
            .map(value_utils::get_single_string);

        let mole_skin = value
            .get(pre_blast_action_ids::MOLE_SKINE)
            .map(value_utils::get_single_string);

        let post_where = value
            .get(pre_blast_action_ids::WHERE_POST)
            .map(value_utils::get_blast_where_value)
            .unwrap_or_default();

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

pub async fn convert_to_message(db_pool: &PgPool, post: PreBlastPost) -> PostMessageRequest {
    let channel_id = match &post.post_where {
        BlastWhere::AoChannel => post.ao.channel_id().to_string(),
        BlastWhere::CurrentChannel(id) => id.to_string(),
    };

    let user = if let Some(id) = post.get_first_q() {
        get_user_by_slack_id(db_pool, &id).await.unwrap_or_default()
    } else {
        None
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
        .section_markdown(&format!(
            "*Moleskine*: {}",
            post.mole_skin.unwrap_or_default()
        ))
        .divider();

    if let Some(f3_user) = user {
        PostMessageRequest::new_as_user(&channel_id, block_builder.blocks, f3_user)
    } else {
        PostMessageRequest::new(&channel_id, block_builder.blocks)
    }
}
