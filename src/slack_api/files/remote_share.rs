pub mod request {
    use crate::slack_api::api_endpoints::FILES_REMOTE_SHARE;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct FileRemoteShareRequest {
        pub channels: String,
        pub file: String,
    }

    impl FileRemoteShareRequest {
        pub fn new(channels: Vec<String>, file: &str) -> Self {
            FileRemoteShareRequest {
                channels: channels.join(","),
                file: file.to_string(),
            }
        }
    }

    impl SlackUrlRequest for FileRemoteShareRequest {
        fn get_api_url(&self) -> &str {
            FILES_REMOTE_SHARE
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct FileRemoteShareResponse {
        pub ok: bool,
        pub error: Option<String>,
    }
}
