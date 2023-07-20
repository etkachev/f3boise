use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_back_blast_by_id;
use crate::db::queries::users::get_db_users;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::ViewModal;
use crate::web_api_routes::slash_commands::back_blast::back_blast_post;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::modal_utils::{
    back_blast_types_list, default_back_blast_type, default_post_option, where_to_post_list,
};
use sqlx::PgPool;

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
) -> Result<(Vec<String>, Vec<String>), AppError> {
    let db_users = get_db_users(db_pool).await?;
    let results = back_blast.get_pax().iter().fold(
        (Vec::<String>::new(), Vec::<String>::new()),
        |mut acc, pax| {
            // don't include qs
            if !back_blast.qs.contains(pax) {
                let matched_db_user = db_users.iter().any(|(_, user)| &user.name == pax);
                if matched_db_user {
                    acc.0.push(pax.to_string());
                } else {
                    acc.1.push(pax.to_string());
                }
            }
            acc
        },
    );
    Ok(results)
}

pub fn create_edit_modal(
    channel_id: &str,
    back_blast: &BackBlastData,
    pax: (Vec<String>, Vec<String>),
    id: &str,
) -> ViewModal {
    let qs = back_blast.qs.clone().into_iter().collect::<Vec<String>>();
    let fngs = back_blast.fngs.clone().into_iter().collect::<Vec<String>>();
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
            Some(pax.0),
            false,
        )
        .plain_input(
            "List untaggable PAX separated by commas (not including FNGs)",
            back_blast_post::back_blast_post_action_ids::UNTAGGABLE_PAX,
            Some("Non-Slackers".to_string()),
            Some(pax.1.join(", ")),
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
