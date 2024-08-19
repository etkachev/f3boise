pub mod request {
    use crate::slack_api::api_endpoints::FILES_COMPLETE_UPLOAD_URL_EXTERNAL;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Default, Debug)]
    pub struct CompleteUploadUrlExternalRequest {
        /// Channel ID where the file will be shared. If not specified the file will be private
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel_id: Option<String>,
        /// Array of file ids and their corresponding (optional) titles
        pub files: Vec<CompleteUploadFileItem>,
        /// The message text introducing the file in specified channels
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initial_comment: Option<String>,
        /// Provide another message's ts value to upload this file as a reply. Never use a reply's ts value; use its parent instead. eg: "1524523204.000192"
        #[serde(skip_serializing_if = "Option::is_none")]
        pub thread_ts: Option<String>,
    }

    impl CompleteUploadUrlExternalRequest {
        /// new single file upload
        pub fn new_single(file_id: &str, title: Option<String>) -> Self {
            let files = if let Some(title) = title {
                vec![CompleteUploadFileItem::new(file_id).with_title(&title)]
            } else {
                vec![CompleteUploadFileItem::new(file_id)]
            };
            CompleteUploadUrlExternalRequest {
                files,
                ..Default::default()
            }
        }

        /// specify which channel id to upload for
        pub fn for_channel(mut self, channel_id: &str) -> Self {
            self.channel_id = Some(channel_id.to_string());
            self
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CompleteUploadFileItem {
        /// file id to complete. eg: "F044GKUHN9Z"
        pub id: String,
        /// optional title to go with file upload
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
    }

    impl CompleteUploadFileItem {
        pub fn new(file_id: &str) -> Self {
            CompleteUploadFileItem {
                id: file_id.to_string(),
                title: None,
            }
        }

        pub fn with_title(mut self, title: &str) -> Self {
            self.title = Some(title.to_string());
            self
        }
    }

    impl SlackUrlRequest for CompleteUploadUrlExternalRequest {
        fn get_api_url(&self) -> &str {
            FILES_COMPLETE_UPLOAD_URL_EXTERNAL
        }
    }
}

pub mod response {
    use super::request::CompleteUploadFileItem;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct CompleteUploadUrlExternalResponse {
        pub ok: bool,
        pub files: Option<Vec<CompleteUploadFileItem>>,
        pub error: Option<String>,
    }
}
