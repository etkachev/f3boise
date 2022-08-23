use crate::web_api_state::AppState;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub mod channel_message;
pub mod event_wrapper;
pub mod slack_challenge;

type HmacSha256 = Hmac<Sha256>;

const CHALLENGE_VERSION: &str = "v0";

fn verify(message: &[u8], code: &str, key: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message);

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let r2 = hex::encode(&code_bytes);
    println!("r2: {}", r2);
    println!("code: {}", code);
    r2 == code
}

fn verify_events_request(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    body: &web::Json<event_wrapper::EventWrapper>,
) -> bool {
    if let Some(time_stamp_header) = req.headers().get("X-Slack-Request-Timestamp") {
        let time_stamp = time_stamp_header.to_str().unwrap_or_else(|_| "");
        // TODO verify time
        let body_string = serde_qs::to_string(&body).unwrap_or_else(|_| "".to_string());
        let sig_base_string = format!("{}:{}:{}", CHALLENGE_VERSION, time_stamp, body_string);
        if let Some(slack_signature_header) = req.headers().get("X-Slack-Signature") {
            let slack_signature = slack_signature_header.to_str().unwrap_or_else(|_| "");
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
    return false;
}

#[post("/events")]
pub async fn slack_events(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<event_wrapper::EventWrapper>,
) -> impl Responder {
    let valid_requset = verify_events_request(&data, &req, &body);
    if !valid_requset {
        return HttpResponse::Unauthorized().body("Event not allowed");
    }

    println!("Body: {:?}", body);

    let response = slack_challenge::ChallengeResponse {
        challenge: body.challenge.clone(),
    };

    return HttpResponse::Ok().json(response);
}
