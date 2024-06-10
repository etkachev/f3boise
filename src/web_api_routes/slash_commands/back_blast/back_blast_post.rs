use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::{BackBlastData, BackBlastType};
use crate::db::queries::users::get_user_by_slack_id;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::slack_api::chat::update_message::request::UpdateMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::interactive_events::interaction_types::InteractionTypes;
use crate::web_api_routes::slack_events::event_times::EventTimes;
use crate::web_api_routes::slash_commands::modal_utils::{value_utils, BlastWhere};
use chrono::NaiveDate;
use sqlx::PgPool;
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
    pub const BB_TYPE: &str = "back_blast_type.select";
    pub const FILE: &str = "file.input";
}

#[derive(Debug)]
pub struct BackBlastPost {
    pub title: String,
    pub date: NaiveDate,
    pub ao: AO,
    pub qs: HashSet<String>,
    pub pax: HashSet<String>,
    pub non_slack_pax: HashSet<String>,
    pub fngs: HashSet<String>,
    pub mole_skine: String,
    pub blast_where: BlastWhere,
    pub bb_type: BackBlastType,
}

impl BackBlastPost {
    /// parse to slack list of users
    pub fn qs_list(&self) -> String {
        self.qs
            .iter()
            .map(|q| format!("<@{}>", q))
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// get full list of pax (slack, non-slack, and fngs) as comma separated string.
    pub fn pax_list(&self) -> String {
        let mut pax = self
            .pax
            .iter()
            .map(|item| format!("<@{}>", item))
            .collect::<HashSet<String>>();
        pax.extend(self.non_slack_pax());
        pax.extend(self.fng_list());
        pax.into_iter().collect::<Vec<String>>().join(", ")
    }

    /// get the first q (to post message as)
    pub fn get_first_q(&self) -> Option<String> {
        self.qs
            .iter()
            .map(|q| q.to_string())
            .collect::<Vec<String>>()
            .first()
            .map(|q| q.to_string())
    }

    /// fng list string with comma separated.
    pub fn fng_string_list(&self) -> String {
        self.fng_list()
            .into_iter()
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// get total pax count
    pub fn pax_count(&self) -> usize {
        let mut pax = self.pax.clone();
        pax.extend(self.non_slack_pax.clone());
        pax.extend(self.qs.clone());
        pax.extend(self.fngs.clone());
        pax.len()
    }

    pub fn saved_context_str(&self, is_valid: bool) -> String {
        let bb_type = self.bb_type.to_string();
        let valid_context_text = if is_valid { "Saved" } else { "Did not save" };

        format!("{} {}", valid_context_text, bb_type)
    }

    /// get list of fngs with names trimmed.
    fn fng_list(&self) -> HashSet<String> {
        self.fngs
            .iter()
            .filter_map(|item| {
                let item = item.trim();
                // filter out fngs names None.
                if !matches!(item, "none" | "None") {
                    Some(item.to_string())
                } else {
                    None
                }
            })
            .collect::<HashSet<String>>()
    }

    /// get non-slack pax with names trimmed.
    fn non_slack_pax(&self) -> HashSet<String> {
        self.non_slack_pax
            .iter()
            .map(|item| item.trim().to_string())
            .collect::<HashSet<String>>()
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

        let slack_pax = value
            .get(back_blast_post_action_ids::PAX)
            .map(value_utils::get_hash_set_strings_from_multi)
            .unwrap_or_default();

        let non_slack_pax = value
            .get(back_blast_post_action_ids::UNTAGGABLE_PAX)
            .map(value_utils::get_hash_set_from_single_comma_split)
            .unwrap_or_default();

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

        let bb_type = value
            .get(back_blast_post_action_ids::BB_TYPE)
            .map(value_utils::get_back_blast_type)
            .unwrap_or_default();

        BackBlastPost {
            title,
            date,
            ao,
            qs,
            pax: slack_pax,
            non_slack_pax,
            fngs,
            mole_skine,
            blast_where,
            bb_type,
        }
    }
}

/// convert to back blast data for saving. pass in hashmap of slack id to pax name
pub fn convert_to_bb_data(
    request: &BackBlastPost,
    users: HashMap<String, String>,
) -> BackBlastData {
    let qs = request
        .qs
        .iter()
        .fold(HashSet::<String>::new(), |mut acc, q| {
            if let Some(name) = users.get(q.as_str()) {
                acc.insert(name.to_string());
            } else {
                acc.insert(q.to_string());
            }
            acc
        });

    let mut pax = request
        .pax
        .iter()
        .fold(HashSet::<String>::new(), |mut acc, item| {
            if let Some(name) = users.get(item.as_str()) {
                acc.insert(name.to_string());
            } else {
                acc.insert(item.to_string());
            }
            acc
        });

    pax.extend(request.non_slack_pax.clone());
    pax.extend(request.fngs.clone());

    let bb_type = request.bb_type.clone();

    let mut data = BackBlastData::new(request.ao.clone(), qs, pax, request.date).with_type(bb_type);

    // setting for it to exist, but not valid.
    data.set_event_times(EventTimes::new("temp".to_string(), "temp".to_string()));
    data.title = Some(request.title.to_string());
    data.moleskine = Some(request.mole_skine.to_string());
    data.fngs.clone_from(&request.fngs);
    data
}

/// convert to update message request
pub fn convert_to_update_message(
    post: BackBlastPost,
    saved: bool,
    id: Option<String>,
    ts: &str,
) -> UpdateMessageRequest {
    let channel_id = match &post.blast_where {
        BlastWhere::AoChannel => post.ao.channel_id().to_string(),
        BlastWhere::CurrentChannel(id) => id.to_string(),
    };

    let block_builder = get_block_builder(post, id, saved);

    UpdateMessageRequest::new(&channel_id, ts, block_builder.blocks)
}

/// convert to post message request. pass in id of saved backblast in db
pub async fn convert_to_message(
    post: BackBlastPost,
    db_pool: &PgPool,
    is_valid: bool,
    id: Option<String>,
) -> PostMessageRequest {
    let channel_id = match &post.blast_where {
        BlastWhere::AoChannel => post.ao.channel_id().to_string(),
        BlastWhere::CurrentChannel(id) => id.to_string(),
    };

    let user = if let Some(id) = post.get_first_q() {
        get_user_by_slack_id(db_pool, &id).await.unwrap_or_default()
    } else {
        None
    };

    let block_builder = get_block_builder(post, id, is_valid);

    if let Some(user) = user {
        PostMessageRequest::new_as_user(&channel_id, block_builder.blocks, user)
    } else {
        PostMessageRequest::new(&channel_id, block_builder.blocks)
    }
}

fn get_block_builder(post: BackBlastPost, id: Option<String>, is_valid: bool) -> BlockBuilder {
    let first_section = format!(
        "*Slackblast*:\n
{}\n
*DATE*: {}\n
*AO*: <#{}>\n
*Q(s)*: {}\n
*PAX*: {}\n
*FNGs*: {}\n
*COUNT*: {}",
        post.title,
        post.date,
        post.ao.channel_id(),
        post.qs_list(),
        post.pax_list(),
        post.fng_string_list(),
        post.pax_count()
    );

    let mut block_builder = BlockBuilder::new()
        .section_markdown(&first_section)
        .divider()
        .section_markdown(post.mole_skine.as_str());

    if let Some(id) = id {
        let interaction_btn = InteractionTypes::new_edit_back_blast(id.as_str());
        block_builder.add_btn(
            "Edit Backblast",
            interaction_btn.to_string().as_str(),
            "edit-backblast",
        );
    }
    let valid_context_text = post.saved_context_str(is_valid);
    block_builder.add_context(valid_context_text.as_str());
    block_builder
}

pub fn get_first_message_section(post: &BackBlastPost) -> String {
    format!(
        "*Slackblast*:\n
{}\n
*DATE*: {}\n
*AO*: <#{}>\n
*Q(s)*: {}\n
*PAX*: {}\n
*FNGs*: {}\n
*COUNT*: {}",
        post.title,
        post.date,
        post.ao.channel_id(),
        post.qs_list(),
        post.pax_list(),
        post.fng_string_list(),
        post.pax_count()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_back_blast_command() {
        let post = BackBlastPost {
            title: "Title".to_string(),
            date: NaiveDate::from_ymd_opt(2023, 3, 2).unwrap(),
            ao: AO::Bleach,
            qs: HashSet::from(["Stinger".to_string()]),
            pax: HashSet::from(["Freighter".to_string(), "Backslash".to_string()]),
            non_slack_pax: HashSet::from([]),
            fngs: HashSet::from(["Fng".to_string(), "None".to_string()]),
            mole_skine: "The Thang".to_string(),
            blast_where: BlastWhere::AoChannel,
            bb_type: BackBlastType::BackBlast,
        };

        assert!(post.pax_list().contains("Fng"));
        assert!(!post.pax_list().contains("none"));
        assert!(!post.pax_list().contains("None"));
    }

    #[test]
    fn double_down_context_str() {
        let post = BackBlastPost {
            title: "Title".to_string(),
            date: NaiveDate::from_ymd_opt(2023, 3, 2).unwrap(),
            ao: AO::Bleach,
            qs: HashSet::from(["Stinger".to_string()]),
            pax: HashSet::from(["Freighter".to_string(), "Backslash".to_string()]),
            non_slack_pax: HashSet::from([]),
            fngs: HashSet::from(["Fng".to_string(), "None".to_string()]),
            mole_skine: "The Thang".to_string(),
            blast_where: BlastWhere::AoChannel,
            bb_type: BackBlastType::DoubleDown,
        };

        let context_str = post.saved_context_str(true);
        assert_eq!(context_str.as_str(), "Saved doubledown");
    }
}
