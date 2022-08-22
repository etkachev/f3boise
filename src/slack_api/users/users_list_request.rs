use crate::slack_api::api_endpoints;
use crate::slack_api::url_requests::SlackUrlRequest;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct UsersListRequest {
    cursor: Option<String>,
    include_locale: Option<bool>,
    limit: Option<u16>,
    team_id: Option<u16>,
}

impl SlackUrlRequest for UsersListRequest {
    fn get_url_request(&self, base_api: &str) -> Url {
        let params = serde_qs::to_string(self).unwrap_or_else(|_| "".to_string());
        Url::parse(format!("{}{}?{}", base_api, api_endpoints::USERS_LIST, params).as_str())
            .unwrap_or_else(|_| Url::parse(base_api).unwrap())
    }
}

impl Default for UsersListRequest {
    fn default() -> Self {
        UsersListRequest {
            cursor: None,
            include_locale: None,
            limit: Some(1000),
            team_id: None,
        }
    }
}
