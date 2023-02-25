use crate::app_state::MutableAppState;
use crate::shared::common_errors::AppError;
use crate::shared::constants;
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::web_api_routes::interactive_events::interaction_payload::{
    Action, ActionChannel, ActionType, ActionUser, BlockAction, ButtonAction,
    InteractionMessageTypes, InteractionPayload, OverflowAction, ViewSubmissionPayload,
    ViewSubmissionPayloadView,
};
use crate::web_api_routes::interactive_events::interaction_types::{
    ActionComboData, InteractionTypes,
};
use crate::web_api_routes::interactive_events::q_line_up::{
    clear_and_update_message, process_q_line_up_event, update_existing_q_line_up_message,
};
use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post::{
    convert_to_message, PreBlastPost,
};
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod interaction_payload;
pub mod interaction_types;
pub mod message_utils;
pub mod q_line_up;
pub mod view_submission;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventBody {
    pub payload: String,
}

fn parse_payload(payload: &str) -> Result<InteractionPayload, AppError> {
    let result: InteractionPayload = serde_json::from_str(payload)?;
    Ok(result)
}

/// consume interactive events from slack
pub async fn interactive_events(
    web_state: web::Data<MutableWebState>,
    app_state: web::Data<MutableAppState>,
    db_pool: web::Data<PgPool>,
    body: web::Form<EventBody>,
) -> impl Responder {
    println!("{:?}", body);
    if let Ok(body) = parse_payload(body.payload.as_str()) {
        match body {
            InteractionPayload::BlockActions(BlockAction {
                actions,
                user,
                channel: action_channel,
                message,
                ..
            }) if !actions.is_empty() => {
                println!("With actions");
                match &actions[0] {
                    ActionType::Button(ButtonAction { action, .. }) => {
                        println!("Button action");
                        if let Err(err) = match_on_button_action(
                            &db_pool,
                            &app_state,
                            &web_state,
                            action,
                            &user,
                            &message,
                            &action_channel,
                        )
                        .await
                        {
                            return HttpResponse::BadRequest().body(err.to_string());
                        }
                    }
                    ActionType::Overflow(OverflowAction {
                        selected_option,
                        action,
                    }) => {
                        println!("Overflow action");
                        if let Err(err) = match_on_overflow_action(
                            &db_pool,
                            &app_state,
                            &web_state,
                            &action_channel,
                            action,
                            selected_option,
                            &message,
                        )
                        .await
                        {
                            return HttpResponse::BadRequest().body(err.to_string());
                        }
                    }
                }
            }
            InteractionPayload::ViewSubmission(ViewSubmissionPayload { user, view }) => {
                match view {
                    ViewSubmissionPayloadView::Modal(modal) => {
                        let form_values = modal.state.get_values();
                        let post = PreBlastPost::from(form_values);
                        println!("from user {:?}", user.username);
                        let message = convert_to_message(post);
                        if let Err(err) = web_state.post_message(message).await {
                            println!("error posting pre blast");
                            println!("{:?}", err);
                            return HttpResponse::BadRequest().body(err.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    } else {
        println!("Could not parse payload");
    }
    HttpResponse::Ok().finish()
}

async fn match_on_button_action(
    db_pool: &PgPool,
    app_state: &MutableAppState,
    web_state: &MutableWebState,
    action: &Action,
    user: &ActionUser,
    message: &Option<InteractionMessageTypes>,
    action_channel: &Option<ActionChannel>,
) -> Result<(), AppError> {
    let action_combo = ActionComboData::from(action.action_id.as_str());
    match &action_combo {
        ActionComboData {
            interaction_type: InteractionTypes::QLineUp,
            ..
        } => {
            println!("q line up action");
            let users = {
                let app = app_state.app.lock().expect("Could not lock app");
                app.get_slack_id_map()
            };

            let slack_id = user.id.as_str();

            // f3 name
            let user = users
                .get(slack_id)
                .map(|name| name.to_string())
                .unwrap_or_else(String::new);

            if user.is_empty() {
                return Err(AppError::General("Could not find user".to_string()));
            }

            if let Ok(channel_id) = get_channel_id_from_action(&action_combo, app_state) {
                process_q_line_up_event(db_pool, &action_combo, vec![user.to_string()], channel_id)
                    .await?;
                println!("succeeded saving q line up!");
                update_existing_q_line_up_message(
                    web_state,
                    action_channel,
                    message,
                    action,
                    &action_combo,
                    slack_id,
                )
                .await?;
            } else {
                return Err(AppError::General("Could not find ao".to_string()));
            }
        }
        ActionComboData {
            interaction_type: InteractionTypes::Unknown,
            ..
        } => {
            println!("Unknown interaction");
        }
    }
    Ok(())
}

async fn match_on_overflow_action(
    db_pool: &PgPool,
    app_state: &MutableAppState,
    web_state: &MutableWebState,
    action_channel: &Option<ActionChannel>,
    action: &Action,
    selected_option: &OptionElement,
    message: &Option<InteractionMessageTypes>,
) -> Result<(), AppError> {
    let action_combo = ActionComboData::from(action.action_id.as_str());
    match selected_option.value.as_str() {
        // when canceling existing q line up
        constants::Q_LINE_UP_CANCEL_TEXT => {
            let channel_id = get_channel_id_from_action(&action_combo, app_state)?;
            clear_and_update_message(
                db_pool,
                web_state,
                &action_combo,
                channel_id.as_str(),
                action_channel,
                message,
                action,
            )
            .await?;
        }
        _ => println!("Unknown overflow option"),
    }
    Ok(())
}

fn get_channel_id_from_action(
    action_combo: &ActionComboData,
    app_state: &MutableAppState,
) -> Result<String, AppError> {
    let channel = PublicChannels::from(&action_combo.ao);
    let channel_id = {
        let app = app_state.app.lock().expect("Could not lock app");
        let all_channels = &app.channels;
        all_channels
            .get(&channel)
            .map(|c| c.id.to_string())
            .unwrap_or_else(String::new)
    };

    if channel_id.is_empty() {
        Err(AppError::General("Could not find ao".to_string()))
    } else {
        Ok(channel_id)
    }
}
