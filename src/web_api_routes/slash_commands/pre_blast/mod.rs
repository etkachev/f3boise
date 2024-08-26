use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::views::payload::{ViewModal, ViewPayload};
use crate::slack_api::views::request::ViewsOpenRequest;
use crate::web_api_routes::slash_commands::modal_utils::view_ids::ViewIds;
use crate::web_api_routes::slash_commands::modal_utils::{default_post_option, where_to_post_list};
use crate::web_api_state::MutableWebState;
use chrono::{Datelike, Duration, NaiveDate};

pub mod pre_blast_post;

pub async fn generate_modal(
    trigger_id: &str,
    web_app: &MutableWebState,
    channel_id: &str,
    user_id: &str,
) -> Result<(), AppError> {
    let modal = create_pre_blast_modal(channel_id, user_id);
    let request = ViewsOpenRequest::new(trigger_id, ViewPayload::Modal(modal));
    web_app.open_view(request).await?;
    Ok(())
}

fn create_pre_blast_modal(channel_id: &str, user_id: &str) -> ViewModal {
    let ao = AO::from_channel_id(channel_id);
    let next_date = get_next_ao_date(&ao);
    let default_time = ao
        .default_time(&next_date.weekday())
        .map(|time| time.format("%H:%M").to_string());
    let ao_equipment = ao
        .ao_type()
        .equipment()
        .into_iter()
        .map(OptionElement::from)
        .collect::<Vec<OptionElement>>();

    // build blocks
    let block_builder = BlockBuilder::new()
        .plain_input(
            "Title",
            pre_blast_post::pre_blast_action_ids::TITLE,
            Some("Snarky Title".to_string()),
            None,
            false,
        )
        .channel_select(
            "AO",
            pre_blast_post::pre_blast_action_ids::AO_SELECT,
            Some(channel_id.to_string()),
            false,
        )
        .date_picker(
            "Workout Date",
            pre_blast_post::pre_blast_action_ids::DATE,
            Some(next_date.to_string()),
            false,
        )
        .time_picker(
            "Workout Time",
            pre_blast_post::pre_blast_action_ids::TIME_SELECT,
            default_time,
            false,
        )
        .multi_users_select(
            "The Q(s)",
            pre_blast_post::pre_blast_action_ids::QS,
            Some(vec![user_id.to_string()]),
            false,
        )
        .divider()
        .plain_input(
            "The Why",
            pre_blast_post::pre_blast_action_ids::WHY,
            None,
            None,
            false,
        )
        .multi_select(
            "Equipment",
            pre_blast_post::pre_blast_action_ids::EQUIPMENT,
            equipment_list(),
            Some(ao_equipment),
            true,
        )
        .plain_input(
            "Other Equipment",
            pre_blast_post::pre_blast_action_ids::OTHER_EQUIPMENT,
            Some(String::from("Anything else to bring?")),
            None,
            true,
        )
        .plain_input(
            "FNGs",
            pre_blast_post::pre_blast_action_ids::FNGS,
            None,
            Some("Always".to_string()),
            true,
        )
        .divider()
        .text_box(
            "The Moleskine",
            pre_blast_post::pre_blast_action_ids::MOLE_SKINE,
            None,
            None,
            true,
        )
        .file_input(
            "Upload Image",
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

    ViewModal::new("Pre Blast", block_builder, "Submit", ViewIds::PreBlast)
}

fn get_next_ao_date(ao: &AO) -> NaiveDate {
    let now = local_boise_time().date_naive();
    let mut current_date_check = now + Duration::days(1);
    // Only check up to a week
    while current_date_check.signed_duration_since(now).num_days() < 7 {
        if ao.week_days().contains(&current_date_check.weekday()) {
            return current_date_check;
        }
        current_date_check += Duration::days(1);
    }

    // Default to tomorrow if not found
    now + Duration::days(1)
}

pub fn equipment_list() -> Vec<OptionElement> {
    vec![
        OptionElement::from(AoEquipment::Coupons),
        OptionElement::from(AoEquipment::Sandbag),
        OptionElement::from(AoEquipment::Ruck),
        OptionElement::from(AoEquipment::WeightVest),
        OptionElement::from(AoEquipment::HeartRateMonitor),
        OptionElement::from(AoEquipment::RunningShoes),
        OptionElement::from(AoEquipment::StopWatch),
        OptionElement::from(AoEquipment::Headlamp),
    ]
}
