pub mod request {
    use crate::slack_api::{api_endpoints, url_requests::SlackUrlRequest};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct ChannelHistoryRequest {
        channel: String,
        cursor: Option<String>,
        include_all_metadata: Option<bool>,
        inclusive: Option<bool>,
        limit: Option<u16>,
    }

    impl ChannelHistoryRequest {
        pub fn new(channel: &str) -> Self {
            ChannelHistoryRequest {
                channel: channel.to_string(),
                ..Default::default()
            }
        }
    }

    impl SlackUrlRequest for ChannelHistoryRequest {
        fn get_api_url(&self) -> &str {
            api_endpoints::CONVERSATION_HISTORY
        }
    }

    impl Default for ChannelHistoryRequest {
        fn default() -> Self {
            ChannelHistoryRequest {
                channel: String::new(),
                cursor: None,
                include_all_metadata: Some(true),
                inclusive: Some(true),
                limit: Some(100),
            }
        }
    }
}

pub mod response {
    use crate::slack_api::cursor_data::CursorData;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelsHistoryResponse {
        pub ok: bool,
        pub messages: Option<Vec<MessageData>>,
        pub error: Option<String>,
        pub has_more: Option<bool>,
        pub pin_count: Option<u16>,
        pub response_metadata: Option<CursorData>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MessageData {
        // full raw string,
        pub text: String,
        // timestamp
        pub ts: String,
        // user id that posted.
        pub user: String,
        pub reactions: Option<Vec<MessageReaction>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MessageReaction {
        pub name: String,
        pub users: Vec<String>,
        pub count: u16,
    }
}
