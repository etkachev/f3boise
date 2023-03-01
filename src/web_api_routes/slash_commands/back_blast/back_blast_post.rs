use crate::app_state::ao_data::AO;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::slash_commands::modal_utils::{value_utils, BlastWhere};
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};

pub mod back_blast_post_action_ids {
    pub const TITLE: &str = "title.input";
    pub const AO: &str = "ao.select";
    pub const DATE: &str = "date.select";
    pub const QS: &str = "qs.select";
    pub const PAX: &str = "pax.select";
    pub const UNTAGGABLE_PAX: &str = "untaggable-pax.input";
    pub const FNGS: &str = "fngs.input";
    pub const MOLESKINE: &str = "moleskine.textarea";
    pub const WHERE_TO_POST: &str = "where-post.select";
}

#[derive(Debug)]
pub struct BackBlastPost {
    pub title: String,
    pub date: NaiveDate,
    pub ao: AO,
    pub qs: HashSet<String>,
    pub pax: HashSet<String>,
    pub fngs: HashSet<String>,
    pub mole_skine: String,
    pub blast_where: BlastWhere,
}

impl BackBlastPost {
    /// parse to slack list of users
    pub fn qs_list(&self) -> String {
        self.qs
            .iter()
            .map(|q| format!("<@{}>", q))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn pax_list(&self) -> String {
        let mut pax = self
            .pax
            .iter()
            .map(|item| format!("<@{}>", item))
            .collect::<Vec<String>>();
        let fngs = self.fng_list();
        pax.extend(fngs);
        pax.join(",")
    }

    pub fn fng_string_list(&self) -> String {
        self.fng_list().join(",")
    }

    pub fn pax_count(&self) -> usize {
        let mut pax = self.pax.clone();
        pax.extend(self.qs.clone());
        pax.extend(self.fngs.clone());
        pax.len()
    }

    fn fng_list(&self) -> Vec<String> {
        self.fngs
            .iter()
            .map(|item| item.trim().to_string())
            .collect::<Vec<String>>()
    }
}

impl From<HashMap<String, BasicValue>> for BackBlastPost {
    fn from(value: HashMap<String, BasicValue>) -> Self {
        let title = value_utils::get_value(
            &value,
            back_blast_post_action_ids::TITLE,
            value_utils::get_single_string,
        )
        .unwrap_or_else(|| String::from("Title"));

        let date = value_utils::get_value(
            &value,
            back_blast_post_action_ids::DATE,
            value_utils::get_single_date,
        )
        .unwrap_or_default();

        let ao = value_utils::get_value(
            &value,
            back_blast_post_action_ids::AO,
            value_utils::get_ao_value,
        )
        .unwrap_or_else(|| AO::Unknown("Not Parsed".to_string()));

        let qs = value
            .get(back_blast_post_action_ids::QS)
            .map(value_utils::get_hash_set_strings_from_multi)
            .unwrap_or_default();

        let mut slack_pax = value
            .get(back_blast_post_action_ids::PAX)
            .map(value_utils::get_hash_set_strings_from_multi)
            .unwrap_or_default();

        let non_slack_pax = value
            .get(back_blast_post_action_ids::UNTAGGABLE_PAX)
            .map(value_utils::get_hash_set_from_single_comma_split)
            .unwrap_or_default();

        slack_pax.extend(non_slack_pax);

        let fngs = value
            .get(back_blast_post_action_ids::FNGS)
            .map(value_utils::get_hash_set_from_single_comma_split)
            .unwrap_or_default();

        let mole_skine = value
            .get(back_blast_post_action_ids::MOLESKINE)
            .map(value_utils::get_single_string)
            .unwrap_or_default();

        let blast_where = value
            .get(back_blast_post_action_ids::WHERE_TO_POST)
            .map(value_utils::get_blast_where_value)
            .unwrap_or_default();

        BackBlastPost {
            title,
            date,
            ao,
            qs,
            pax: slack_pax,
            fngs,
            mole_skine,
            blast_where,
        }
    }
}

pub fn convert_to_message(post: BackBlastPost) -> PostMessageRequest {
    let channel_id = match &post.blast_where {
        BlastWhere::AoChannel => post.ao.channel_id().to_string(),
        BlastWhere::CurrentChannel(id) => id.to_string(),
        // TODO
        BlastWhere::Myself => "TODO".to_string(),
    };

    let first_section = format!(
        "*Slackblast*:\
{}\
*DATE*: {}\
*AO*: <#{}>\
*Q(s)*: {}\
*PAX*: {}\
*FNGs*: {}\
*COUNT*: {}",
        post.title,
        post.date,
        post.ao.channel_id(),
        post.qs_list(),
        post.pax_list(),
        post.fng_string_list(),
        post.pax_count()
    );

    let block_builder = BlockBuilder::new()
        .section_markdown(&first_section)
        .divider()
        .section_markdown(post.mole_skine.as_str());
    PostMessageRequest::new(&channel_id, block_builder.blocks)
}
