use crate::app_state::MutableAppState;
use crate::db::save_q_line_up::{map_from_action, save_list};
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::{BlockBuilder, SectionBlock};
use crate::slack_api::channels::public_channels::PublicChannels;
use crate::web_api_routes::interactive_events::interaction_payload::{
    ActionType, BlockAction, ButtonAction, InteractionPayload,
};
use crate::web_api_routes::interactive_events::interaction_types::{
    ActionComboData, InteractionTypes,
};
use crate::web_api_routes::slash_commands::SlashCommandForm;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod interaction_payload;
pub mod interaction_types;

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
    _data: web::Data<MutableWebState>,
    app_state: web::Data<MutableAppState>,
    db_pool: web::Data<PgPool>,
    body: web::Form<EventBody>,
) -> impl Responder {
    println!("{:?}", body);
    if let Ok(body) = parse_payload(body.payload.as_str()) {
        match body {
            InteractionPayload::BlockActions(BlockAction { actions, user, .. })
                if !actions.is_empty() =>
            {
                println!("With actions");
                match &actions[0] {
                    ActionType::Button(ButtonAction { action, .. }) => {
                        println!("Button action");
                        let action_combo = ActionComboData::from(action.action_id.as_str());
                        match &action_combo {
                            ActionComboData {
                                interaction_type: InteractionTypes::QLineUp,
                                ..
                            } => {
                                println!("q line up action");
                                let user = {
                                    let app = app_state.app.lock().expect("Could not lock app");
                                    app.users
                                        .get(user.id.as_str())
                                        .map(|user| user.name.to_string())
                                        .unwrap_or_else(String::new)
                                };

                                if user.is_empty() {
                                    return HttpResponse::BadRequest().body("Could not find user");
                                }

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
                                    return HttpResponse::BadRequest().body("Could not find ao");
                                }

                                match process_q_line_up_event(
                                    &db_pool,
                                    action_combo,
                                    vec![user],
                                    channel_id,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        println!("succeeded saving q line up!");
                                    }
                                    Err(err) => {
                                        return HttpResponse::BadRequest().body(err.to_string())
                                    }
                                }
                            }
                            ActionComboData {
                                interaction_type: InteractionTypes::Unknown,
                                ..
                            } => {
                                println!("Unknown interaction");
                            }
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

async fn process_q_line_up_event(
    db_pool: &PgPool,
    action: ActionComboData,
    users: Vec<String>,
    channel_id: String,
) -> Result<(), AppError> {
    let action = map_from_action(&action, users, channel_id)?;
    let list = vec![action];
    save_list(db_pool, &list).await?;
    Ok(())
}

pub async fn test_btn_message(
    _db_pool: web::Data<PgPool>,
    _app_state: web::Data<MutableAppState>,
    web_state: web::Data<MutableWebState>,
    form: web::Form<SlashCommandForm>,
) -> impl Responder {
    // TODO add guard of some sort?
    if web_state.verify_token != form.token {
        return HttpResponse::Unauthorized().body("Sorry buddy");
    }

    match form.command.as_str() {
        "/btn-test" => match test_btn() {
            Ok(block) => HttpResponse::Ok().json(block),
            Err(err) => HttpResponse::Ok().body(err.to_string()),
        },
        _ => HttpResponse::Ok().body("Unknown command"),
    }
}

fn test_btn() -> Result<BlockBuilder, AppError> {
    let block_builder = BlockBuilder::new()
        .header(":calendar:  Q Line-up")
        .context("*November 2022*  |  Fill em up!")
        .divider()
        .section_markdown(":calendar: |   *UPCOMING EVENTS*  | :calendar:")
        .section(SectionBlock::new_markdown_with_btn(
            "`11/05` *Bleach*",
            "Sign up",
            "11/05::bleach",
        ))
        .divider();
    Ok(block_builder)
}
