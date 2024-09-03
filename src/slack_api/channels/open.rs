//! api for opening conversation with direction message single or multiple users

/// module for requesting api call
pub mod request {
    use crate::slack_api::api_endpoints::CONVERSATION_OPEN;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct OpenConversationRequest {
        /// Resume a conversation by supplying an im or mpim's ID. Or provide the users field instead
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        /// Do not create a direct message or multi-person direct message. This is used to see if there is an existing dm or mpdm
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prevent_creation: Option<bool>,
        /// Boolean, indicates you want the full IM channel definition in the response
        #[serde(skip_serializing_if = "Option::is_none")]
        pub return_im: Option<bool>,
        /// Comma separated lists of users. If only one user is included, this creates a 1:1 DM.
        /// The ordering of the users is preserved whenever a multi-person direct message is returned.
        /// Supply a channel when not supplying users
        #[serde(skip_serializing_if = "Option::is_none")]
        pub users: Option<String>,
    }

    impl OpenConversationRequest {
        pub fn new(users: &[String]) -> Self {
            OpenConversationRequest {
                users: Some(users.join(",")),
                channel: None,
                prevent_creation: None,
                return_im: None,
            }
        }
    }

    impl SlackUrlRequest for OpenConversationRequest {
        fn get_api_url(&self) -> &str {
            CONVERSATION_OPEN
        }
    }
}

/// module for response on api call
pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct OpenConversationResponse {
        pub ok: bool,
        pub no_op: Option<bool>,
        pub already_open: Option<bool>,
        pub channel: Option<OpenConversationChannel>,
        pub error: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct OpenConversationChannel {
        /// example D069C7QFK. Id of channel to use in order to post new message
        pub id: String,
        pub created: Option<usize>,
        pub is_im: Option<bool>,
        pub is_org_shared: Option<bool>,
        pub user: Option<String>,
        /// time stamp
        pub last_read: Option<String>,
        pub unread_count: Option<usize>,
        pub unread_count_display: Option<usize>,
        pub is_open: Option<bool>,
    }
}
