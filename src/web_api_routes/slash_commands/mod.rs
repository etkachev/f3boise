use crate::app_state::MutableAppState;
use crate::web_api_routes::slash_commands::my_stats::handle_my_stats;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod my_stats;

/// respond to slash commands
pub async fn my_stats_command_route(
    db_pool: web::Data<PgPool>,
    app_state: web::Data<MutableAppState>,
    form: web::Form<SlashCommandForm>,
) -> impl Responder {
    println!("form: {:?}", form);
    match form.command.as_str() {
        "/my-stats" => match handle_my_stats(&db_pool, &app_state, &form).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        _ => HttpResponse::Ok().body("Unknown command"),
    }
}

#[derive(Deserialize, Debug)]
pub struct SlashCommandForm {
    pub token: String,
    pub channel_id: String,
    pub channel_name: Option<String>,
    pub user_id: String,
    pub user_name: Option<String>,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub api_app_id: String,
}

#[derive(Serialize)]
pub struct SlashCommandResponse {
    blocks: Vec<SlackBlock>,
}

impl SlashCommandResponse {
    pub fn new(blocks: Vec<SlackBlock>) -> Self {
        SlashCommandResponse { blocks }
    }
}

#[derive(Serialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub event_type: String,
    pub text: SlackBlockText,
}

impl SlackBlock {
    pub fn new(text: &str) -> Self {
        SlackBlock {
            event_type: "section".to_string(),
            text: SlackBlockText {
                event_type: "mrkdwn".to_string(),
                text: text.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct SlackBlockText {
    #[serde(rename = "type")]
    pub event_type: String,
    pub text: String,
}
