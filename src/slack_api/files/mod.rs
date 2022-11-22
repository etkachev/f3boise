/// request structs and methods for uploading file to slack channel(s)
pub mod request {
    use crate::slack_api::api_endpoints::FILES_UPLOAD;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct FileUploadRequest {
        pub channels: String,
        pub file: Vec<u8>,
        pub filename: Option<String>,
        pub filetype: Option<String>,
        pub initial_comment: Option<String>,
        pub title: Option<String>,
    }

    impl FileUploadRequest {
        pub fn new(channels: Vec<String>, file: Vec<u8>, filename: &str, comment: &str) -> Self {
            FileUploadRequest {
                channels: channels.join(","),
                file,
                filename: Some(filename.to_string()),
                filetype: None,
                initial_comment: Some(comment.to_string()),
                title: None,
            }
        }

        /// get form request for this file upload. TODO trait extraction?
        pub fn get_form_request(&self) -> reqwest::multipart::Form {
            let part = reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("test.png")
                .mime_str("image/png")
                .unwrap();
            let form = reqwest::multipart::Form::new()
                .text("channels", self.channels.to_string())
                .text(
                    "filename",
                    self.filename
                        .as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| String::from("unknown")),
                )
                .text(
                    "initial_comment",
                    self.initial_comment
                        .as_ref()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| String::from("unknown comment")),
                )
                .part("file", part);
            form
        }
    }

    impl SlackUrlRequest for FileUploadRequest {
        fn get_api_url(&self) -> &str {
            FILES_UPLOAD
        }
    }
}

/// response structs
pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct FileUploadResponse {
        pub ok: bool,
        pub file: Option<FileUploaded>,
        pub error: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct FileUploaded {
        pub id: String,
        pub title: String,
    }
}
