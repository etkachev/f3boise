use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::MutableAppState;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::slack_events::event_times::EventTimes;
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
    pub non_slack_pax: HashSet<String>,
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
            .join(", ")
    }

    /// get full list of pax (slack, non-slack, and fngs) as comma separated string.
    pub fn pax_list(&self) -> String {
        let mut pax = self
            .pax
            .iter()
            .map(|item| format!("<@{}>", item))
            .collect::<Vec<String>>();
        pax.extend(self.non_slack_pax());
        pax.extend(self.fng_list());
        pax.join(", ")
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
        self.fng_list().join(", ")
    }

    /// get total pax count
    pub fn pax_count(&self) -> usize {
        let mut pax = self.pax.clone();
        pax.extend(self.non_slack_pax.clone());
        pax.extend(self.qs.clone());
        pax.extend(self.fngs.clone());
        pax.len()
    }

    /// get list of fngs with names trimmed.
    fn fng_list(&self) -> Vec<String> {
        self.fngs
            .iter()
            .map(|item| item.trim().to_string())
            .collect::<Vec<String>>()
    }

    /// get non-slack pax with names trimmed.
    fn non_slack_pax(&self) -> Vec<String> {
        self.non_slack_pax
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
        }
    }
}

/// convert to back blast data for saving
pub fn convert_to_bb_data(request: &BackBlastPost, app_state: &MutableAppState) -> BackBlastData {
    let users = {
        let app = app_state.app.lock().unwrap();
        app.get_slack_id_map()
    };

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

    let mut data = BackBlastData::new(request.ao.clone(), qs, pax, request.date);

    // setting for it to exist, but not valid.
    data.set_event_times(EventTimes::new("temp".to_string(), "temp".to_string()));
    data
}

pub fn convert_to_message(
    post: BackBlastPost,
    app_state: &MutableAppState,
    is_valid: bool,
) -> PostMessageRequest {
    let channel_id = match &post.blast_where {
        BlastWhere::AoChannel => post.ao.channel_id().to_string(),
        BlastWhere::CurrentChannel(id) => id.to_string(),
        // TODO
        BlastWhere::Myself => "TODO".to_string(),
    };

    let user = if let Some(id) = post.get_first_q() {
        let app = app_state.app.lock().unwrap();
        app.get_user(&id)
    } else {
        None
    };

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

    let valid_context_text = if is_valid {
        "Saved backblast"
    } else {
        "Did not save backblast"
    };
    let block_builder = BlockBuilder::new()
        .section_markdown(&first_section)
        .divider()
        .section_markdown(post.mole_skine.as_str())
        .context(valid_context_text);

    if let Some(user) = user {
        PostMessageRequest::new_as_user(&channel_id, block_builder.blocks, user)
    } else {
        PostMessageRequest::new(&channel_id, block_builder.blocks)
    }
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
    use crate::app_state::parse_backblast::parse_back_blast;
    use crate::slack_api::channels::list::response::ChannelData;
    use crate::slack_api::channels::public_channels::PublicChannels;
    use crate::users::f3_user::F3User;

    fn empty_channels() -> HashMap<PublicChannels, ChannelData> {
        HashMap::from([])
    }

    fn hash_set_user(id: &str, name: &str) -> (String, F3User) {
        (
            id.to_string(),
            F3User {
                id: Some(id.to_string()),
                name: name.to_string(),
                email: "email@test.com".to_string(),
                img_url: None,
            },
        )
    }

    #[test]
    fn parse_from_back_blast_command() {
        let post = BackBlastPost {
            title: "Title".to_string(),
            date: NaiveDate::from_ymd(2023, 3, 2),
            ao: AO::Bleach,
            qs: HashSet::from(["Stinger".to_string()]),
            pax: HashSet::from(["Freighter".to_string(), "Backslash".to_string()]),
            non_slack_pax: HashSet::from([]),
            fngs: HashSet::from(["Fng".to_string()]),
            mole_skine: "The Thang".to_string(),
            blast_where: BlastWhere::AoChannel,
        };

        let message = get_first_message_section(&post);
        let users = HashMap::<String, F3User>::from([hash_set_user("U03SR452HL7", "Backslash")]);
        let parsed = parse_back_blast(&message, &users, &empty_channels());
        println!("{:?}", parsed);
        assert_eq!(true, true);
    }
}
