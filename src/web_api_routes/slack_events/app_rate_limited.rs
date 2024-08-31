use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_state::MutableWebState;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// This event type is only dispatched when your app is rate limited on the Events API.
/// Rate limiting currently occurs when your app would receive more than 30,000 events in an hour from a single workspace
/// This event does not require a specific OAuth scope or subscription. You'll automatically receive it when your app's event subscriptions are rate limited or disabled.
//
// Event subscriptions may be limited and disabled when your app does not respond with a HTTP 200 OK to at 5% of event deliveries in the past 60 minutes.
#[derive(Serialize, Deserialize, Debug)]
pub struct AppRateLimitedData {
    /// Subscriptions between your app and the workspace with this ID are being rate limited
    pub team_id: Option<String>,
    /// Your application's ID, especially useful if you have multiple applications working with the Events API.
    pub api_app_id: String,
    /// A rounded epoch time value indicating the minute your application became rate limited for this workspace. 1518467820 is at 2018-02-12 20:37:00 UTC
    pub minute_rate_limited: usize,
}

pub async fn handle_app_rate_limited(web_state: &MutableWebState, data: &AppRateLimitedData) {
    let when = DateTime::from_timestamp(data.minute_rate_limited as i64, 0).unwrap_or_default();
    let additional_data = format!(
        "Additional data: when rate limit reached - {}",
        when.naive_utc()
    );
    let block_builder = BlockBuilder::new()
        .header("Yo I'm poor...")
        .section_markdown(
            "Slack let me know that we have reached our app rate limit on events consumed.",
        )
        .section_markdown("This happens when we receive more than 30,000 events in an hour")
        .divider()
        .section_markdown(&additional_data);
    if let Err(err) = web_state
        .post_message(PostMessageRequest::new("C0728DNRDD0", block_builder.blocks))
        .await
    {
        println!(
            "Issues with sending message about app rate limit. - {:?}",
            err
        );
    }
}
