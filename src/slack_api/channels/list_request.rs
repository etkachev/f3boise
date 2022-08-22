use super::types::ChannelTypes;
use crate::slack_api::{api_endpoints, url_requests::SlackUrlRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use url::Url;

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
    fn get_url_request(&self, base_api: &str) -> Url {
        let params = serde_qs::to_string(self).unwrap_or_else(|_| "".to_string());
        Url::parse(
            format!(
                "{}{}?{}",
                base_api,
                api_endpoints::CONVERSATION_LISTS,
                params
            )
            .as_str(),
        )
        .unwrap_or_else(|_| Url::parse(base_api).unwrap())
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
