pub mod request {
    use crate::slack_api::api_endpoints::CONVERSATION_INVITE;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;
    use std::collections::HashMap;

    /// request object for inviting users to channel.
    #[derive(Serialize)]
    pub struct InviteToConvoRequest {
        pub channel: String,
        pub users: String,
    }

    impl InviteToConvoRequest {
        pub fn new(
            channel: &str,
            users: HashMap<String, String>,
            already_in_channel: Vec<String>,
        ) -> Self {
            let users: Vec<String> = users
                .keys()
                .filter_map(|key| {
                    if !already_in_channel.contains(key) {
                        Some(key.to_string())
                    } else {
                        None
                    }
                })
                .collect();
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
