use crate::slack_api::{api_endpoints, url_requests::SlackUrlRequest};
use serde::{Deserialize, Serialize};
use url::Url;

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
    fn get_url_request(&self, base_api: &str) -> Url {
        let params = serde_qs::to_string(self).unwrap_or_else(|_| "".to_string());
        Url::parse(
            format!(
                "{}{}?{}",
                base_api,
                api_endpoints::CONVERSATION_HISTORY,
                params
            )
            .as_str(),
        )
        .unwrap_or_else(|_| Url::parse(base_api).unwrap())
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
