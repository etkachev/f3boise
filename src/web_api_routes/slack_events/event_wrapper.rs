use super::channel_message::ChannelMessageEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{formats::Flexible, TimestampSeconds};

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct EventWrapper {
    pub token: String,
    #[serde(rename = "type")]
    pub event_type: String,
    /// may come sometimes for challenging events api
    pub challenge: Option<String>,
    pub event_id: Option<String>,
    #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
    pub event_time: Option<DateTime<Utc>>,
    pub event: Option<EventTypes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EventTypes {
    Message(ChannelMessageEvent),
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_event() {
        let raw_json = r#"
        {
            "token": "one-long-verification-token",
            "team_id": "T061EG9R6",
            "api_app_id": "A0PNCHHK2",
            "event": {
                "type": "message",
                "channel": "C024BE91L",
                "user": "U2147483697",
                "text": "Live long and prospect.",
                "ts": "1355517523.000005",
                "event_ts": "1355517523.000005",
                "channel_type": "channel"
            },
            "type": "event_callback",
            "authed_teams": [
                "T061EG9R6"
            ],
            "event_id": "Ev0PV52K21",
            "event_time": 1355517523
        }
        "#;

        let result: EventWrapper = serde_json::from_str(raw_json).unwrap();
        assert_eq!(result.event_type, "event_callback".to_string());
    }
}
