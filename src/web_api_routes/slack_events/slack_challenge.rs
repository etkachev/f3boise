use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChallengeEventPayload {
    #[serde(rename = "type")]
    pub event_type: String,
    pub token: String,
    pub challenge: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ChallengeResponse {
    pub challenge: Option<String>,
}
