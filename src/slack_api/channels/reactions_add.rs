pub mod request {
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
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct ReactionsAddResponse {
        pub ok: bool,
        pub error: Option<String>,
    }
}
