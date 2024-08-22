pub mod complete_upload_url_external;
pub mod files_list;
pub mod get_upload_url_external;

/// request structs and methods for uploading file to slack channel(s)
pub mod request {
    pub struct FileUpload {
        pub channel_id: String,
        pub file: Vec<u8>,
        pub filename: String,
        pub mimetype: String,
        pub title: Option<String>,
    }

    impl FileUpload {
        pub fn new(channel_id: &str, file: Vec<u8>, filename: &str, mimetype: &str) -> Self {
            FileUpload {
                channel_id: channel_id.to_string(),
                file,
                filename: filename.to_string(),
                mimetype: mimetype.to_string(),
                title: None,
            }
        }

        pub fn with_title(mut self, title: &str) -> Self {
            self.title = Some(title.to_string());
            self
        }

        /// get multi-part form
        pub fn get_form_request(&self) -> reqwest::multipart::Form {
            let part = reqwest::multipart::Part::bytes(self.file.clone())
                .file_name(self.filename.to_string())
                .mime_str(self.mimetype.as_str())
                .unwrap();
            reqwest::multipart::Form::new()
                .text("filename", self.filename.to_string())
                .part("file", part)
        }
    }
}
