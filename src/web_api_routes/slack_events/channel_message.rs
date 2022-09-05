use crate::app_state::backblast_data::{BACK_BLAST_TAG, SLACK_BLAST_TAG};
use crate::app_state::parse_backblast::parse_back_blast;
use crate::app_state::MutableAppState;
use crate::db::save_back_blast;
use crate::slack_api::channels::reactions_add::request::channel_request;
use crate::web_api_routes::slack_events::event_times::EventTimes;
use crate::web_api_state::MutableWebState;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelMessageEvent {
    /**
    "subtype": "bot_message",
    "ts": "1358877455.000010",
    "text": "Pushing is the answer",
    "bot_id": "BB12033",
    "username": "github",
    "icons": {}

    channel: "id"
    channel_type: "channel"
    event_ts: "1662333021.006700",
    hidden: true,
    message: "B040Z8BJUSE",
    icons: {},
    subtype: "bot_message",
    text: "FULL"
    ts: string,
    type: "message"
    username: "Stringer via backblast"


    **/
    pub channel: String,
    // TODO update type
    pub subtype: Option<String>,
    /// when user message
    pub user: Option<String>,
    /// when bot message
    pub username: Option<String>,
    pub text: String,
    pub ts: String,
    pub event_ts: String,
    // TODO update type
    pub channel_type: String,
}

pub async fn handle_channel_message(
    event: &ChannelMessageEvent,
    web_app: &MutableWebState,
    app_state: &MutableAppState,
    db_pool: &PgPool,
) {
    let (first_line, _) = event.text.split_once('\n').unwrap_or(("", ""));
    match first_line.to_lowercase() {
        // is back-blast
        line if line.starts_with(BACK_BLAST_TAG) || line.starts_with(SLACK_BLAST_TAG) => {
            let mut data = {
                let app = app_state.app.lock().unwrap();
                parse_back_blast(event.text.as_str(), &app.users, &app.channels)
            };
            data.set_event_times(EventTimes::new(
                event.ts.to_string(),
                event.event_ts.to_string(),
            ));

            let verified = data.is_valid_back_blast();
            let channel_request = channel_request(&data, verified, event.channel.as_str());
            web_app.back_blast_verified(channel_request).await;
            if verified {
                if let Err(err) = save_back_blast::save(db_pool, &[data]).await {
                    println!("Error saving bb to db: {:?}", err);
                }
            }
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn time_format() {
        use serde::Deserialize;
        use serde_with::{formats::Flexible, TimestampSeconds};

        #[serde_with::serde_as]
        #[derive(Deserialize, Debug)]
        struct S {
            #[serde_as(as = "TimestampSeconds<String, Flexible>")]
            time: DateTime<Utc>,
        }

        let my_s = serde_json::from_str::<S>(r#"{ "time": 1355517523.000005 }"#).unwrap();
        assert!(!my_s.time.to_string().is_empty());
    }
}
