use crate::app_state::MutableAppState;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::PgPool;

mod app_rate_limited;
pub mod channel_message;
pub mod emoji_reactions;
pub mod event_times;
pub mod event_wrapper;
pub mod slack_challenge;
pub mod team_join;
mod user_profile_changed;

type HmacSha256 = Hmac<Sha256>;

const CHALLENGE_VERSION: &str = "v0";

fn verify(message: &[u8], code: &str, key: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message);

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let r2 = hex::encode(code_bytes);
    r2 == code
}

fn verify_events_request(
    data: &web::Data<MutableWebState>,
    req: &HttpRequest,
    body: &web::Json<event_wrapper::EventWrapper>,
) -> bool {
    if let Some(time_stamp_header) = req.headers().get("X-Slack-Request-Timestamp") {
        let time_stamp = time_stamp_header.to_str().unwrap_or("");
        // TODO verify time
        let body_string = serde_qs::to_string(&body).unwrap_or_else(|_| "".to_string());
        let sig_base_string = format!("{}:{}:{}", CHALLENGE_VERSION, time_stamp, body_string);
        if let Some(slack_signature_header) = req.headers().get("X-Slack-Signature") {
            let slack_signature = slack_signature_header.to_str().unwrap_or("");
            let slack_signing_secret = &data.signing_secret;
            let valid = verify(
                sig_base_string.as_bytes(),
                slack_signature,
                slack_signing_secret.as_bytes(),
            );

            if valid || body.token == data.verify_token {
                return true;
            }
        }
    }
    false
}

const DIVIDER: &str = "\n\n========\n\n";

pub async fn slack_events(
    web_state: web::Data<MutableWebState>,
    app_state: web::Data<MutableAppState>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<event_wrapper::EventWrapper>,
) -> impl Responder {
    println!("Event incoming!");
    // TODO refactor
    let valid_request = verify_events_request(&web_state, &req, &body);
    if !valid_request {
        return HttpResponse::Unauthorized().body("Event not allowed");
    }

    println!("{}Body: {:?}{}", DIVIDER, body, DIVIDER);

    if let Some(event) = &body.event {
        match event {
            event_wrapper::EventTypes::Message(message_data) => {
                channel_message::handle_channel_message(
                    message_data,
                    &web_state,
                    &app_state,
                    &db_pool,
                )
                .await;
            }
            event_wrapper::EventTypes::TeamJoin(join_data) => {
                team_join::handle_new_user(&db_pool, &join_data.user, &app_state, &web_state).await;
            }
            event_wrapper::EventTypes::ReactionAdded(reaction_data) => {
                emoji_reactions::handle_reaction_add(&db_pool, reaction_data, &app_state).await;
            }
            event_wrapper::EventTypes::ReactionRemoved(reaction_data) => {
                emoji_reactions::handle_reaction_remove(&db_pool, reaction_data, &app_state).await;
            }
            event_wrapper::EventTypes::AppRateLimited(rate_data) => {
                app_rate_limited::handle_app_rate_limited(&web_state, rate_data).await;
            }
            event_wrapper::EventTypes::UserProfileChanged(user_data) => {
                user_profile_changed::handle_user_profile_changed(&db_pool, user_data).await;
            }
            _ => (),
        }
    }

    let response = slack_challenge::ChallengeResponse {
        challenge: body.challenge.clone(),
    };

    HttpResponse::Ok().json(response)
}
