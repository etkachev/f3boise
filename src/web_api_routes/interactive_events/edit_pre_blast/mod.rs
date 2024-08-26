use crate::app_state::pre_blast_data::PreBlastData;
use crate::db::queries::pre_blasts;
use crate::db::queries::users::get_db_users;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::ViewModal;
use crate::users::f3_user::F3User;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::modal_utils::{default_post_option, where_to_post_list};
use crate::web_api_routes::slash_commands::pre_blast::{equipment_list, pre_blast_post};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

pub async fn get_pre_blast_data(db: &PgPool, id: &str) -> Result<PreBlastData, AppError> {
    let item = pre_blasts::get_pre_blast_by_id(db, id).await?;

    if let Some(db_entry) = item {
        let data = PreBlastData::from(db_entry);

        Ok(data)
    } else {
        Err(AppError::from("Missing Preblast in database"))
    }
}

pub async fn get_pre_blast_user_data(
    db: &PgPool,
    data: &PreBlastData,
) -> Result<PreBlastUsersEdit, AppError> {
    let db_users = get_db_users(db).await?;
    let results = get_slack_user_split(data, db_users);
    Ok(results)
}

#[derive(Debug, Default)]
pub struct PreBlastUsersEdit {
    slack_users: Vec<F3User>,
}

impl PreBlastUsersEdit {
    pub fn convert_to_slack_ids(&self, users: &HashSet<String>) -> Vec<String> {
        users.iter().fold(Vec::<String>::new(), |mut acc, item| {
            if let Some(id) = self.get_slack_id(item) {
                acc.push(id);
            }
            acc
        })
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
    pre_blast: &PreBlastData,
    db_users: HashMap<String, F3User>,
) -> PreBlastUsersEdit {
    pre_blast
        .qs
        .iter()
        .fold(PreBlastUsersEdit::default(), |mut acc, qs| {
            let matched_db_user = db_users
                .iter()
                .find(|(_, user)| user.name.to_lowercase() == qs.to_lowercase());
            if matched_db_user.is_some() {
                acc.slack_users
                    .push(matched_db_user.map(|(_, user)| user.clone()).unwrap());
            }
            acc
        })
}

pub fn create_edit_modal(
    channel_id: &str,
    pre_blast: &PreBlastData,
    edit_data: PreBlastUsersEdit,
    id: &str,
) -> ViewModal {
    let qs = edit_data.convert_to_slack_ids(&pre_blast.qs);
    let block_builder = BlockBuilder::new()
        .plain_input(
            "Title",
            pre_blast_post::pre_blast_action_ids::TITLE,
            Some("Snarky Title".to_string()),
            Some(pre_blast.title.to_string()),
            false,
        )
        .channel_select(
            "AO",
            pre_blast_post::pre_blast_action_ids::AO_SELECT,
            Some(pre_blast.ao.channel_id().to_string()),
            false,
        )
        .date_picker(
            "Workout Date",
            pre_blast_post::pre_blast_action_ids::DATE,
            Some(pre_blast.date.to_string()),
            false,
        )
        .time_picker(
            "Workout Time",
            pre_blast_post::pre_blast_action_ids::TIME_SELECT,
            Some(pre_blast.start_time.to_string()),
            false,
        )
        .multi_users_select(
            "The Q(s)",
            pre_blast_post::pre_blast_action_ids::QS,
            Some(qs),
            false,
        )
        .divider()
        .plain_input(
            "The Why",
            pre_blast_post::pre_blast_action_ids::WHY,
            None,
            Some(pre_blast.why.to_string()),
            false,
        )
        .multi_select(
            "Equipment",
            pre_blast_post::pre_blast_action_ids::EQUIPMENT,
            equipment_list(),
            Some(pre_blast.official_equipment_option_elements()),
            true,
        )
        .plain_input(
            "Other Equipment",
            pre_blast_post::pre_blast_action_ids::OTHER_EQUIPMENT,
            Some(String::from("Anything else to bring?")),
            Some(pre_blast.other_equipment_option_elements()),
            true,
        )
        .plain_input(
            "FNGs",
            pre_blast_post::pre_blast_action_ids::FNGS,
            None,
            pre_blast.fng_message.clone(),
            true,
        )
        .divider()
        .text_box(
            "The Moleskine",
            pre_blast_post::pre_blast_action_ids::MOLE_SKINE,
            None,
            pre_blast.mole_skin.clone(),
            true,
        )
        .img_ids(
            pre_blast.img_ids.iter().map(|id| id.to_string()).collect(),
            "Current Images",
        )
        .file_input(
            "Re-Upload Image",
            pre_blast_post::pre_blast_action_ids::FILE,
            vec!["jpg", "jpeg", "png", "gif"],
            true,
        )
        .select(
            "Choose where to post this",
            pre_blast_post::pre_blast_action_ids::WHERE_POST,
            where_to_post_list(channel_id),
            Some(default_post_option(Some(channel_id))),
            false,
        )
        .context("Please wait after hitting submit!");
    ViewModal::new("Pre Blast", block_builder, "Update", ViewIds::PreBlastEdit)
        .with_private_meta(id)
}
