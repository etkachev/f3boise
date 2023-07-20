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
        /// (Legacy) Pass true to post the message as the authed user instead of as a bot. Defaults to false. Can only be used by classic Slack apps.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub as_user: Option<bool>,
        /// Emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon_emoji: Option<String>,
        /// URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon_url: Option<String>,
        /// Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below
        #[serde(skip_serializing_if = "Option::is_none")]
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
                as_user: None,
                icon_url: user.img_url.clone(),
                icon_emoji: None,
                username: Some(format!("{} (via BoiseBot)", user.name)),
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
        /// timestamp on when it was posted.
        pub ts: Option<String>,
        pub error: Option<String>,
    }
}
