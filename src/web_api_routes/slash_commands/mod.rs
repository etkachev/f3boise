use crate::app_state::ao_data::AO;
use crate::app_state::MutableAppState;
use crate::shared::time::local_boise_time;
use crate::web_api_routes::back_blast_data::ao_monthly_leaderboard::get_ao_monthly_stats_graph;
use crate::web_api_routes::graphs::ao_pax_leaderboard::post_ao_pax_leaderboard_graph;
use crate::web_api_routes::slash_commands::ao_monthly_stats_graph::AOMonthlyStatsGraphCommand;
use crate::web_api_routes::slash_commands::ao_stats::get_ao_stats_block;
use crate::web_api_routes::slash_commands::invite_all::handle_invite_all;
use crate::web_api_routes::slash_commands::my_stats::handle_my_stats;
use crate::web_api_routes::slash_commands::q_line_up::{
    get_q_line_up_for_ao, get_q_line_up_message_all, send_all_q_line_up_message,
    send_ao_q_line_up_message, QLineUpCommand,
};
use crate::web_api_routes::slash_commands::top_pax::handle_top_pax;
use crate::web_api_routes::slash_commands::wheres_freighter::get_wheres_freighter_message;
use crate::web_api_routes::sync::sync_data_to_state;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

pub mod ao_monthly_stats_graph;
pub mod ao_stats;
pub mod invite_all;
pub mod my_stats;
pub mod q_line_up;
pub mod top_pax;
pub mod wheres_freighter;

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
        "/q-sheet" | "/post-q-sheet" => match QLineUpCommand::from(form.text.as_str()) {
            QLineUpCommand { ao: None, month } => {
                let users = {
                    let app = app_state.app.lock().expect("Could not lock app");
                    app.get_user_name_map()
                };
                let start_date = month
                    .map(|date| date.pred())
                    .unwrap_or_else(|| local_boise_time().date_naive());

                // see if request came from ao channel, then filter to ao data only, otherwise all

                let possible_ao = AO::from_channel_id(form.channel_id.as_str());
                let possible_ao = match possible_ao {
                    AO::Unknown(_) => None,
                    _ => Some(possible_ao),
                };

                match form.command.as_str() {
                    // this will actually post to slack and be visible to everyone
                    "/post-q-sheet" => {
                        if let Some(ao) = possible_ao {
                            match send_ao_q_line_up_message(
                                &db_pool,
                                ao,
                                &start_date,
                                &users,
                                form.channel_id.as_str(),
                                &web_state,
                            )
                            .await
                            {
                                Ok(_) => HttpResponse::Ok().body("Posting Q Line up"),
                                Err(_) => HttpResponse::BadRequest().body("Invalid command"),
                            }
                        } else {
                            match send_all_q_line_up_message(
                                &db_pool,
                                &start_date,
                                &users,
                                form.channel_id.as_str(),
                                &web_state,
                            )
                            .await
                            {
                                Ok(_) => HttpResponse::Ok().body("Posting Q Line up"),
                                Err(_) => HttpResponse::BadRequest().body("Invalid command"),
                            }
                        }
                    }
                    // this will be the silent response where only the requester will see.
                    "/q-sheet" => {
                        if let Some(ao) = possible_ao {
                            match get_q_line_up_for_ao(&db_pool, ao, &start_date, &users).await {
                                Ok(builder) => HttpResponse::Ok().json(builder),
                                Err(err) => HttpResponse::BadRequest().body(err.to_string()),
                            }
                        } else {
                            match get_q_line_up_message_all(&db_pool, &start_date, &users).await {
                                Ok(builder) => HttpResponse::Ok().json(builder),
                                Err(err) => HttpResponse::BadRequest().body(err.to_string()),
                            }
                        }
                    }
                    _ => HttpResponse::BadRequest().body("Unknown command"),
                }
            }
            QLineUpCommand {
                ao: Some(ao),
                month,
            } => {
                let users = {
                    let app = app_state.app.lock().expect("Could not lock app");
                    app.get_user_name_map()
                };
                let start_date = month
                    .map(|date| date.pred())
                    .unwrap_or_else(|| local_boise_time().date_naive());
                match send_ao_q_line_up_message(
                    &db_pool,
                    ao,
                    &start_date,
                    &users,
                    form.channel_id.as_str(),
                    &web_state,
                )
                .await
                {
                    Ok(_) => HttpResponse::Ok().body("Posting Q Line Up"),
                    Err(_) => HttpResponse::BadRequest().body("Invalid command"),
                }
            }
        },
        "/top-pax" => match handle_top_pax(&db_pool, form.text.as_str()).await {
            Ok(builder) => HttpResponse::Ok().json(builder),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        "/resync-bot" => match sync_data_to_state(&db_pool, &web_state, &app_state).await {
            Ok(()) => HttpResponse::Ok().body("Re-synced Boise bot"),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        "/ao-stats" => match get_ao_stats_block(&db_pool, &form).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        "/wheres-freighter" => match get_wheres_freighter_message(&db_pool).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        "/ao-month-graph" => {
            let command = AOMonthlyStatsGraphCommand::new(form.text.as_str());
            match get_ao_monthly_stats_graph(
                &db_pool,
                &Some(command.month),
                &web_state,
                form.channel_id.to_string(),
            )
            .await
            {
                Ok(_) => HttpResponse::Ok().body("Posting Monthly Stats"),
                Err(err) => HttpResponse::Ok().body(err.to_string()),
            }
        }
        "/top-pax-30-days" => {
            match post_ao_pax_leaderboard_graph(&db_pool, &web_state, form.channel_id.to_string())
                .await
            {
                Ok(_) => HttpResponse::Ok().body("Posting Top Pax stats"),
                Err(err) => HttpResponse::Ok().body(err.to_string()),
            }
        }
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
