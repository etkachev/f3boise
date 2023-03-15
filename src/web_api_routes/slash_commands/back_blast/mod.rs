use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::{ViewModal, ViewPayload};
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::modal_utils::{
    back_blast_types_list, default_back_blast_type, default_post_option, where_to_post_list,
};
use crate::web_api_state::MutableWebState;

pub mod back_blast_post;

pub async fn generate_modal(
    trigger_id: &str,
    web_app: &MutableWebState,
    channel_id: &str,
    user_id: &str,
) -> Result<(), AppError> {
    let modal = create_modal(channel_id, user_id);
    let view = ViewsOpenRequest::new(trigger_id, ViewPayload::Modal(modal));
    web_app.open_view(view).await?;
    Ok(())
}

fn create_modal(channel_id: &str, user_id: &str) -> ViewModal {
    let default_date = local_boise_time().date_naive();
    let default_moleskine = r#"*WARMUP:*
*THE THANG:*
*MARY:*
*ANNOUNCEMENTS:*
*COT:*"#;
    let default_moleskine = default_moleskine.to_string();
    let block_builder = BlockBuilder::new()
        .plain_input(
            "Title",
            back_blast_post::back_blast_post_action_ids::TITLE,
            Some("Snarky Title?".to_string()),
            None,
            false,
        )
        .channel_select(
            "The AO",
            back_blast_post::back_blast_post_action_ids::AO,
            Some(channel_id.to_string()),
            false,
        )
        .date_picker(
            "Workout Date",
            back_blast_post::back_blast_post_action_ids::DATE,
            Some(default_date.to_string()),
            false,
        )
        .multi_users_select(
            "The Q(s)",
            back_blast_post::back_blast_post_action_ids::QS,
            Some(vec![user_id.to_string()]),
            false,
        )
        .multi_users_select(
            "The PAX",
            back_blast_post::back_blast_post_action_ids::PAX,
            None,
            false,
        )
        .plain_input(
            "List untaggable PAX separated by commas (not including FNGs)",
            back_blast_post::back_blast_post_action_ids::UNTAGGABLE_PAX,
            Some("Non-Slackers".to_string()),
            None,
            true,
        )
        .plain_input(
            "List FNGs, separated by commas",
            back_blast_post::back_blast_post_action_ids::FNGS,
            Some("FNGs".to_string()),
            None,
            true,
        )
        .text_box(
            "The Moleskine",
            back_blast_post::back_blast_post_action_ids::MOLESKINE,
            Some("Enter BD info".to_string()),
            Some(default_moleskine),
            false,
        ).context("If trying to tag PAX in here, substitute _ for spaces and do not include titles in parenthesis (ie, @Moneyball not @Moneyball_(F3_STC)). Spelling is important, capitalization is not!")
        .select(
            "Backblast type",
            back_blast_post::back_blast_post_action_ids::BB_TYPE,
            back_blast_types_list(),
            Some(default_back_blast_type()),
            false,
        )
        .select(
            "Choose where to post this",
            back_blast_post::back_blast_post_action_ids::WHERE_TO_POST,
            where_to_post_list(channel_id),
            Some(default_post_option()),
            false,
        ).context("Do not hit Submit more than once! Even if you get a timeout error, the backblast has likely already been posted. If using email, this can take time and this form may not automatically close.");
    ViewModal::new("Back Blast", block_builder, "Submit", ViewIds::BackBlast)
}
