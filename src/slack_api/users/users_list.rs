pub mod request {
    use crate::slack_api::api_endpoints;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct UsersListRequest {
        cursor: Option<String>,
        include_locale: Option<bool>,
        limit: Option<u16>,
        team_id: Option<u16>,
    }

    impl SlackUrlRequest for UsersListRequest {
        fn get_api_url(&self) -> &str {
            api_endpoints::USERS_LIST
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
}

pub mod response {
    use crate::slack_api::cursor_data::CursorData;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UsersListResponse {
        pub ok: bool,
        pub members: Option<Vec<SlackUserData>>,
        pub error: Option<String>,
        pub response_metadata: Option<CursorData>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SlackUserData {
        pub id: String,
        pub name: String,
        pub profile: SlackUserProfile,
        pub deleted: bool,
        #[serde(default)]
        pub real_name: String,
        #[serde(default)]
        pub is_admin: bool,
        #[serde(default)]
        pub is_owner: bool,
        #[serde(default)]
        pub is_primary_owner: bool,
        #[serde(default)]
        pub is_restricted: bool,
        #[serde(default)]
        pub is_ultra_restricted: bool,
        pub is_bot: bool,
        pub is_app_user: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SlackUserProfile {
        pub title: String,
        pub phone: String,
        pub real_name: String,
        pub real_name_normalized: String,
        pub display_name: String,
        pub display_name_normalized: String,
        pub email: Option<String>,
        pub first_name: String,
        pub last_name: String,
    }
}
