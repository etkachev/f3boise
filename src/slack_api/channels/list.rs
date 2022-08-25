pub mod request {
    use crate::slack_api::channels::types::ChannelTypes;
    use crate::slack_api::{api_endpoints, url_requests::SlackUrlRequest};
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    #[derive(Deserialize, Serialize)]
    pub struct ConversationListRequest {
        pub exclude_archived: Option<bool>,
        pub limit: Option<u16>,
        pub types: Option<String>,
    }

    impl ConversationListRequest {
        pub fn with_types(types: Vec<ChannelTypes>) -> Self {
            let mut types_hash = HashSet::<String>::new();
            for channel_type in types.iter() {
                types_hash.insert(channel_type.name());
            }

            let types_string = types_hash.into_iter().collect::<Vec<String>>().join(",");

            ConversationListRequest {
                types: if types_string.is_empty() {
                    None
                } else {
                    Some(types_string)
                },
                ..Default::default()
            }
        }
    }

    impl SlackUrlRequest for ConversationListRequest {
        fn get_api_url(&self) -> &str {
            api_endpoints::CONVERSATION_LISTS
        }
    }

    impl Default for ConversationListRequest {
        fn default() -> Self {
            ConversationListRequest {
                exclude_archived: Some(true),
                limit: Some(100),
                types: Some(String::new()),
            }
        }
    }
}

pub mod response {
    use crate::slack_api::cursor_data::CursorData;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelsListResponse {
        pub ok: bool,
        pub channels: Option<Vec<ChannelData>>,
        pub response_metadata: Option<CursorData>,
        pub error: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelData {
        pub id: String,
        pub name: String,
    }
}
