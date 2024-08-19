pub mod request {
    use crate::slack_api::api_endpoints::FILES_GET_UPLOAD_URL_EXTERNAL;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct GetUploadUrlExternalRequest {
        /// Name of the file being uploaded. e.g: laughingoutloudcat.jpg
        pub filename: String,
        /// Size in bytes of the file being uploaded
        pub length: usize,
        /// Description of image for screen-reader
        pub alt_txt: Option<String>,
    }

    impl GetUploadUrlExternalRequest {
        pub fn new(file_name: &str, length: usize) -> Self {
            GetUploadUrlExternalRequest {
                filename: file_name.to_string(),
                length,
                alt_txt: None,
            }
        }
    }

    impl SlackUrlRequest for GetUploadUrlExternalRequest {
        fn get_api_url(&self) -> &str {
            FILES_GET_UPLOAD_URL_EXTERNAL
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct GetUploadUrlExternalResponse {
        pub ok: bool,
        pub upload_url: Option<String>,
        pub file_id: Option<String>,
        pub error: Option<String>,
    }
}
