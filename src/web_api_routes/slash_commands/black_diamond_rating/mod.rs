use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::{ViewModal, ViewPayload};
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_state::MutableWebState;

pub mod black_diamond_rating_post;

pub async fn generate_modal(
    trigger_id: &str,
    web_app: &MutableWebState,
    channel_id: &str,
) -> Result<(), AppError> {
    let modal = create_modal(channel_id);
    let view = ViewsOpenRequest::new(trigger_id, ViewPayload::Modal(modal));
    web_app.open_view(view).await?;
    Ok(())
}

fn create_modal(channel_id: &str) -> ViewModal {
    let block_builder = BlockBuilder::new()
        .plain_input(
            "Number of Pax",
            black_diamond_rating_post::post_ids::PAX_COUNT,
            Some("How many Pax?".to_string()),
            None,
            false,
        )
        .plain_input(
            "Vests Removed",
            black_diamond_rating_post::post_ids::VESTS_REMOVED,
            Some("How many vests removed?".to_string()),
            None,
            false,
        )
        .plain_input(
            "Miles",
            black_diamond_rating_post::post_ids::MILES,
            Some("How many miles?".to_string()),
            None,
            false,
        )
        .plain_input(
            "Avg Heart Rate",
            black_diamond_rating_post::post_ids::AVG_HR,
            Some("Avg heart rate of pax".to_string()),
            None,
            false,
        )
        .channel_select(
            "Where to Post",
            black_diamond_rating_post::post_ids::WHERE_POST,
            Some(channel_id.to_string()),
            false,
        );
    ViewModal::new(
        "Black Diamond Rating",
        block_builder,
        "Submit",
        ViewIds::BlackDiamondRating,
    )
}
