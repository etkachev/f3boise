pub mod request {
    use crate::slack_api::api_endpoints;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ConversationMembersRequest {
        channel: String,
        limit: Option<u16>,
        cursor: Option<String>,
    }

    impl ConversationMembersRequest {
        pub fn new(channel_id: &str) -> Self {
            ConversationMembersRequest {
                channel: channel_id.to_string(),
                ..Default::default()
            }
        }
    }

    impl SlackUrlRequest for ConversationMembersRequest {
        fn get_api_url(&self) -> &str {
            api_endpoints::CONVERSATION_MEMBERS
        }
    }

    impl Default for ConversationMembersRequest {
        fn default() -> Self {
            ConversationMembersRequest {
                channel: String::new(),
                limit: Some(1000),
                cursor: None,
            }
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct ConversationMembersResponse {
        pub ok: bool,
        pub members: Option<Vec<String>>,
        pub error: Option<String>,
    }
}
