pub mod request {
    use crate::slack_api::api_endpoints::CHAT_POST_MESSAGE;
    use crate::slack_api::block_kit::BlockType;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use crate::users::f3_user::F3User;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct PostMessageRequest {
        /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details
        pub channel: String,
        pub blocks: Vec<BlockType>,
        pub as_user: Option<bool>,
        pub icon_emoji: Option<String>,
        pub icon_url: Option<String>,
        pub username: Option<String>,
    }

    impl PostMessageRequest {
        pub fn new(channel: &str, blocks: Vec<BlockType>) -> Self {
            PostMessageRequest {
                channel: channel.to_string(),
                blocks,
                as_user: None,
                icon_url: None,
                icon_emoji: None,
                username: None,
            }
        }

        pub fn new_as_user(channel: &str, blocks: Vec<BlockType>, user: F3User) -> Self {
            PostMessageRequest {
                channel: channel.to_string(),
                blocks,
                as_user: Some(true),
                icon_url: user.img_url.clone(),
                icon_emoji: None,
                username: Some(user.name),
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
        pub channel: Option<String>,
        pub error: Option<String>,
    }
}
