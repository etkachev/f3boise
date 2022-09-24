use crate::app_state::MutableAppState;
use crate::shared::time::local_boise_time;
use crate::web_api_routes::slash_commands::invite_all::handle_invite_all;
use crate::web_api_routes::slash_commands::my_stats::handle_my_stats;
use crate::web_api_routes::slash_commands::q_line_up::{
    get_q_line_up_for_ao, get_q_line_up_message_all, QLineUpCommand,
};
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

pub mod invite_all;
pub mod my_stats;
pub mod q_line_up;

/// respond to slash commands
pub async fn slack_slash_commands_route(
    db_pool: web::Data<PgPool>,
    app_state: web::Data<MutableAppState>,
    web_state: web::Data<MutableWebState>,
    form: web::Form<SlashCommandForm>,
) -> impl Responder {
    // TODO add guard of some sort?
    if web_state.verify_token != form.token {
        return HttpResponse::Unauthorized().body("Sorry buddy");
    }

    println!("form: {:?}", form);
    match form.command.as_str() {
        "/my-stats" => match handle_my_stats(&db_pool, &app_state, &form).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        "/invite-all" => match handle_invite_all(&web_state, &app_state, &form).await {
            Ok(response) => HttpResponse::Ok().body(response),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        // TODO update
        "/btn-test" => match QLineUpCommand::from(form.text.as_str()) {
            QLineUpCommand { ao: None, .. } => {
                let users = {
                    let app = app_state.app.lock().expect("Could not lock app");
                    app.get_user_name_map()
                };
                let now = local_boise_time().date_naive();
                match get_q_line_up_message_all(&db_pool, &now, &users).await {
                    Ok(block_builder) => HttpResponse::Ok().json(block_builder),
                    Err(_) => HttpResponse::BadRequest().body("Invalid command"),
                }
            }
            QLineUpCommand { ao: Some(ao), .. } => {
                let users = {
                    let app = app_state.app.lock().expect("Could not lock app");
                    app.get_user_name_map()
                };
                let now = local_boise_time().date_naive();
                match get_q_line_up_for_ao(&db_pool, ao, &now, &users).await {
                    Ok(block_builder) => HttpResponse::Ok().json(block_builder),
                    Err(_) => HttpResponse::BadRequest().body("Invalid command"),
                }
            }
        },
        _ => {
            println!("command not accepted: {}", form.command);
            HttpResponse::Ok().body("Unknown command")
        }
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
