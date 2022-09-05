pub mod request {
    use crate::app_state::backblast_data::BackBlastData;
    use crate::slack_api::{api_endpoints, url_requests::SlackUrlRequest};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct ReactionsAddRequest {
        /// Channel where message was posted
        pub channel: String,
        /// emoji name
        pub name: String,
        /// timestamp of the message itself
        pub timestamp: String,
    }

    impl ReactionsAddRequest {
        pub fn new(channel: String, emoji: &str, timestamp: String) -> Self {
            ReactionsAddRequest {
                channel,
                name: emoji.to_string(),
                timestamp,
            }
        }
    }

    impl SlackUrlRequest for ReactionsAddRequest {
        fn get_api_url(&self) -> &str {
            api_endpoints::REACTIONS_ADD
        }
    }

    pub fn channel_request(
        back_blast: &BackBlastData,
        verified: bool,
        channel_id: &str,
    ) -> Option<ReactionsAddRequest> {
        if let Some(event_times) = &back_blast.event_times {
            let emoji = if verified { "white_check_mark" } else { "x" };
            return Some(ReactionsAddRequest::new(
                channel_id.to_string(),
                emoji,
                event_times.ts.to_string(),
            ));
        }

        None
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct ReactionsAddResponse {
        pub ok: bool,
        pub error: Option<String>,
    }
}
