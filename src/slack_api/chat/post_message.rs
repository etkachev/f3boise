pub mod request {
    use crate::slack_api::api_endpoints::CHAT_POST_MESSAGE;
    use crate::slack_api::block_kit::BlockType;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct PostMessageRequest {
        /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details
        pub channel: String,
        pub blocks: Vec<BlockType>,
    }

    impl PostMessageRequest {
        pub fn new(channel: &str, blocks: Vec<BlockType>) -> Self {
            PostMessageRequest {
                channel: channel.to_string(),
                blocks,
            }
        }
    }

    impl SlackUrlRequest for PostMessageRequest {
        fn get_api_url(&self) -> &str {
            CHAT_POST_MESSAGE
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct PostMessageResponse {
        pub ok: bool,
        pub channel: String,
        pub error: Option<String>,
    }
}
