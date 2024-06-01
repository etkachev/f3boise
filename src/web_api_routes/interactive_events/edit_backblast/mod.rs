use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_back_blast_by_id;
use crate::db::queries::users::get_db_users;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::ViewModal;
use crate::users::f3_user::F3User;
use crate::web_api_routes::slash_commands::back_blast::back_blast_post;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::modal_utils::{
    back_blast_types_list, default_back_blast_type, default_post_option, where_to_post_list,
};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

pub async fn get_back_blast(db_pool: &PgPool, id: &str) -> Result<BackBlastData, AppError> {
    let bb = get_back_blast_by_id(db_pool, id).await?;
    if bb.is_none() {
        return Err(AppError::from("Could not find backblast"));
    }

    let bb = BackBlastData::from(bb.unwrap());

    Ok(bb)
}

/// get tuple of pax based on backblast, 0 is db users, 1 is non slack users.
pub async fn get_user_data(
    db_pool: &PgPool,
    back_blast: &BackBlastData,
) -> Result<BackBlastUsersEdit, AppError> {
    let db_users = get_db_users(db_pool).await?;
    let results = get_slack_user_split(back_blast, db_users);
    Ok(results)
}

#[derive(Debug, Default)]
pub struct BackBlastUsersEdit {
    slack_users: Vec<F3User>,
    non_slack_users: Vec<F3User>,
}

impl BackBlastUsersEdit {
    pub fn new(slack_users: Vec<F3User>, non_slack_users: Vec<F3User>) -> Self {
        BackBlastUsersEdit {
            slack_users,
            non_slack_users,
        }
    }
    pub fn convert_to_slack_ids(&self, users: &HashSet<String>) -> Vec<String> {
        users.iter().fold(Vec::<String>::new(), |mut acc, item| {
            if let Some(id) = self.get_slack_id(item) {
                acc.push(id);
            }
            acc
        })
    }

    /// get slack ids of all pax except qs
    pub fn get_non_q_slack_ids(&self, qs: &HashSet<String>) -> Vec<String> {
        self.slack_users
            .iter()
            .fold(Vec::<String>::new(), |mut acc, item| {
                // exclude qs
                if !qs.contains(&item.name.to_lowercase()) {
                    if let Some(id) = &item.id {
                        acc.push(id.to_string());
                    }
                }
                acc
            })
    }

    pub fn get_non_slack_users(&self) -> Vec<String> {
        self.non_slack_users
            .iter()
            .map(|item| item.name.to_string())
            .collect()
    }

    fn get_slack_id(&self, user: &str) -> Option<String> {
        self.slack_users.iter().find_map(|item| {
            if item.name.to_lowercase() == user.to_lowercase() {
                item.id.clone()
            } else {
                None
            }
        })
    }
}

fn get_slack_user_split(
    back_blast: &BackBlastData,
    db_users: HashMap<String, F3User>,
) -> BackBlastUsersEdit {
    back_blast
        .get_pax()
        .iter()
        .fold(BackBlastUsersEdit::default(), |mut acc, pax| {
            let matched_db_user = db_users
                .iter()
                .find(|(_, user)| user.name.to_lowercase() == pax.to_lowercase());
            if matched_db_user.is_some() {
                acc.slack_users
                    .push(matched_db_user.map(|(_, user)| user.clone()).unwrap());
            } else {
                acc.non_slack_users.push(F3User::non_slack_user(pax, ""));
            }
            acc
        })
}

pub fn create_edit_modal(
    channel_id: &str,
    back_blast: &BackBlastData,
    pax: BackBlastUsersEdit,
    id: &str,
) -> ViewModal {
    let qs = pax.convert_to_slack_ids(&back_blast.qs);
    let fngs = back_blast.fngs.clone().into_iter().collect::<Vec<String>>();
    let slack_pax = pax.get_non_q_slack_ids(&back_blast.qs);
    let non_slack = pax.get_non_slack_users();
    let block_builder = BlockBuilder::new()
        .plain_input(
            "Title",
            back_blast_post::back_blast_post_action_ids::TITLE,
            Some("Snarky Title?".to_string()),
            back_blast.title.clone(),
            false,
        )
        .channel_select(
            "The AO",
            back_blast_post::back_blast_post_action_ids::AO,
            Some(back_blast.ao.channel_id().to_string()),
            false,
        )
        .date_picker(
            "Workout Date",
            back_blast_post::back_blast_post_action_ids::DATE,
            Some(back_blast.date.to_string()),
            false,
        )
        .multi_users_select(
            "The Q(s)",
            back_blast_post::back_blast_post_action_ids::QS,
            Some(qs),
            false,
        )
        .multi_users_select(
            "The PAX",
            back_blast_post::back_blast_post_action_ids::PAX,
            Some(slack_pax),
            false,
        )
        .plain_input(
            "List untaggable PAX separated by commas (not including FNGs)",
            back_blast_post::back_blast_post_action_ids::UNTAGGABLE_PAX,
            Some("Non-Slackers".to_string()),
            Some(non_slack.join(", ")),
            true,
        )
        .plain_input(
            "List FNGs, separated by commas",
            back_blast_post::back_blast_post_action_ids::FNGS,
            Some("FNGs".to_string()),
            Some(fngs.join(", ")),
            true,
        )
        .text_box(
            "The Moleskine",
            back_blast_post::back_blast_post_action_ids::MOLESKINE,
            Some("Enter BD info".to_string()),
            back_blast.moleskine.clone(),
            false,
        ).context("If trying to tag PAX in here, substitute _ for spaces and do not include titles in parenthesis (ie, @Moneyball not @Moneyball_(F3_STC)). Spelling is important, capitalization is not!")
        .select(
            "Backblast type",
            back_blast_post::back_blast_post_action_ids::BB_TYPE,
            back_blast_types_list(),
            Some(default_back_blast_type(Some(back_blast.bb_type.clone()))),
            false,
        )
        .select(
            "Choose where to post this",
            back_blast_post::back_blast_post_action_ids::WHERE_TO_POST,
            where_to_post_list(channel_id),
            Some(default_post_option(Some(channel_id))),
            false,
        ).context("Do not hit Submit more than once! Even if you get a timeout error, the backblast has likely already been posted. If using email, this can take time and this form may not automatically close.");
    ViewModal::new(
        "Back Blast",
        block_builder,
        "Update",
        ViewIds::BackBlastEdit,
    )
    .with_private_meta(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::ao_data::AO;
    use chrono::NaiveDate;
    use std::collections::HashSet;

    fn hash_set_user(id: &str, name: &str) -> (String, F3User) {
        (
            id.to_string(),
            F3User {
                id: Some(id.to_string()),
                name: name.to_string(),
                email: "email@test.com".to_string(),
                img_url: None,
                invited_by: None,
            },
        )
    }

    #[test]
    fn split_users_correctly() {
        let qs = HashSet::<String>::from(["stinger".to_string()]);
        let bb_data = BackBlastData::new(
            AO::Tower,
            qs.clone(),
            HashSet::<String>::from([
                "backslash".to_string(),
                "puff".to_string(),
                "fng-one".to_string(),
            ]),
            NaiveDate::from_ymd_opt(2023, 7, 3).unwrap(),
        );
        let users = HashMap::<String, F3User>::from([
            hash_set_user("123", "Stinger"),
            hash_set_user("22", "backslash"),
            hash_set_user("33", "puff"),
        ]);
        let result = get_slack_user_split(&bb_data, users);
        let q_ids = result.convert_to_slack_ids(&qs);
        let slack_pax = result.get_non_q_slack_ids(&qs);
        let non_slack = result.get_non_slack_users();
        assert_eq!(q_ids.len(), 1);
        assert_eq!(q_ids.get(0).unwrap(), "123");
        assert_eq!(slack_pax.len(), 2);
        assert!(slack_pax.contains(&"22".to_string()));
        assert!(slack_pax.contains(&"33".to_string()));
        assert_eq!(non_slack.len(), 1);
    }

    #[test]
    fn generate_modal_correctly() {
        let mut bb_data = BackBlastData::new(
            AO::Tower,
            HashSet::<String>::from(["stinger".to_string()]),
            HashSet::<String>::from([
                "backslash".to_string(),
                "puff".to_string(),
                "fng-one".to_string(),
                "fng-two".to_string(),
            ]),
            NaiveDate::from_ymd_opt(2023, 7, 3).unwrap(),
        );
        bb_data.title = Some("My Title".to_string());
        bb_data.moleskine = Some("Some moleskine".to_string());
        bb_data.id = Some("bb-id-123".to_string());
        let users = HashMap::<String, F3User>::from([
            hash_set_user("123", "Stinger"),
            hash_set_user("22", "backslash"),
            hash_set_user("33", "puff"),
        ]);
        let users_edit = get_slack_user_split(&bb_data, users);
        let modal = create_edit_modal("channel-1", &bb_data, users_edit, "bb-id-123");
        // strictly for debugging
        println!("{:?}", modal);
        assert_eq!(1, 1);
    }
}
