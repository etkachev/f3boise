pub mod request {
    use crate::slack_api::api_endpoints::CHAT_UPDATE_MESSAGE;
    use crate::slack_api::block_kit::BlockType;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct UpdateMessageRequest {
        /// Channel containing the message to be updated
        pub channel: String,
        /// Timestamp of the message to be updated
        pub ts: String,
        pub blocks: Option<Vec<BlockType>>,
    }

    impl UpdateMessageRequest {
        pub fn new(channel: &str, ts: &str, blocks: Vec<BlockType>) -> Self {
            UpdateMessageRequest {
                channel: channel.to_string(),
                ts: ts.to_string(),
                blocks: Some(blocks),
            }
        }
    }

    impl SlackUrlRequest for UpdateMessageRequest {
        fn get_api_url(&self) -> &str {
            CHAT_UPDATE_MESSAGE
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UpdateMessageResponse {
        pub ok: bool,
        pub channel: Option<String>,
        pub ts: Option<String>,
        pub error: Option<String>,
    }
}
