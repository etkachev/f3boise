use crate::app_state::MutableAppState;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::interactive_events::interaction_payload::InteractionPayload;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod block_actions;
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
    if let Ok(payload) = parse_payload(body.payload.as_str()) {
        match payload {
            InteractionPayload::BlockActions => {
                match block_actions::handle_block_actions(
                    &body.payload,
                    &db_pool,
                    &app_state,
                    &web_state,
                )
                .await
                {
                    Ok(()) => {
                        println!("Successfully handled block action");
                    }
                    Err(err) => {
                        println!("Error handling block actions {:?}", err);
                        return HttpResponse::BadRequest().body(err.to_string());
                    }
                }
            }
            InteractionPayload::ViewSubmission => {
                println!("parsing view submission");
                match view_submission::handle_view_submission(
                    &body.payload,
                    &web_state,
                    &app_state,
                    &db_pool,
                )
                .await
                {
                    Ok(()) => println!("Successfully handled view submission"),
                    Err(err) => {
                        println!("{:?}", err);
                        return HttpResponse::BadRequest().body(err.to_string());
                    }
                }
            }
        }
    }

    HttpResponse::Ok().finish()
}
