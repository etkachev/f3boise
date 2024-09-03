use crate::shared::responses::{failure, success};
use crate::slack_api::block_kit::BlockBuilder;
use crate::web_api_state::MutableWebState;
use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DMRequest {
    slack_id: String,
    msg: String,
}

/// send direct message to someone on slack
pub async fn send_direct_message_route(
    web_state: web::Data<MutableWebState>,
    req: web::Json<DMRequest>,
) -> impl Responder {
    let block_builder = BlockBuilder::new().section_markdown(&req.msg);
    match web_state
        .send_direct_message(&req.slack_id, block_builder)
        .await
    {
        Ok(_) => success(),
        Err(err) => failure(err),
    }
}
