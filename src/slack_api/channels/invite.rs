pub mod request {
    use crate::slack_api::api_endpoints::CONVERSATION_INVITE;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use crate::users::f3_user::F3User;
    use serde::Serialize;
    use std::collections::HashMap;

    /// request object for inviting users to channel.
    #[derive(Serialize)]
    pub struct InviteToConvoRequest {
        pub channel: String,
        pub users: String,
    }

    impl InviteToConvoRequest {
        pub fn new(channel: &str, users: &HashMap<String, F3User>) -> Self {
            let users: Vec<String> = users.keys().map(|key| key.to_string()).collect();
            InviteToConvoRequest {
                channel: channel.to_string(),
                users: users.join(","),
            }
        }
    }

    impl SlackUrlRequest for InviteToConvoRequest {
        fn get_api_url(&self) -> &str {
            CONVERSATION_INVITE
        }
    }
}

pub mod response {
    use crate::slack_api::channels::list::response::ChannelData;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct InviteToConvoResponse {
        pub ok: bool,
        pub channel: Option<ChannelData>,
        pub error: Option<String>,
    }
}
