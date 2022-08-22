use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ChallengeEventPayload {
    #[serde(rename = "type")]
    event_type: String,
    token: String,
    challenge: String,
}

#[derive(Deserialize, Serialize)]
struct ChallengeResponse {
    challenge: String,
}

#[post("/challenge-event-api")]
pub async fn challenge_event_api() -> impl Responder {
    HttpResponse::Ok()
}
