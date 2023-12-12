pub mod request {
    use crate::slack_api::api_endpoints::CONVERSATION_KICK;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct KickFromChannelRequest {
        /// ID of conversation to remove user from
        pub channel: String,
        /// User ID to be removed
        pub user: String,
    }

    impl KickFromChannelRequest {
        pub fn new(user: &str, channel: &str) -> Self {
            KickFromChannelRequest {
                channel: channel.to_string(),
                user: user.to_string(),
            }
        }
    }

    impl SlackUrlRequest for KickFromChannelRequest {
        fn get_api_url(&self) -> &str {
            CONVERSATION_KICK
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct KickFromChannelResponse {
        pub ok: bool,
        pub error: Option<String>,
    }
}
