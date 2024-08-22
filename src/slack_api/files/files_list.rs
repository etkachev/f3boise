pub mod request {
    use crate::slack_api::api_endpoints::FILES_LIST;
    use crate::slack_api::url_requests::SlackUrlRequest;
    use chrono::FixedOffset;
    use serde::Serialize;

    #[derive(Serialize, Default)]
    pub struct FilesListRequest {
        /// Filter files appearing in a specific channel, indicated by its ID
        pub channel: Option<String>,
        /// Number of items to return per page. default 100
        pub count: Option<usize>,
        /// Page number of results to return. Default 1
        pub page: Option<usize>,
        /// Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit
        pub show_files_hidden_by_limit: Option<bool>,
        /// Filter files created after this timestamp (inclusive)
        pub ts_from: Option<String>,
        /// Filter files created before this timestamp (inclusive)
        pub ts_to: Option<String>,
        /// Filter files by type (see below). You can pass multiple values in the types argument,
        /// like types=spaces,snippets.The default value is all, which does not filter the list.
        /// Default "all"
        pub types: Option<String>,
        /// Filter files created by a single user
        pub user: Option<String>,
    }

    impl FilesListRequest {
        pub fn new(count: usize) -> Self {
            FilesListRequest {
                count: Some(count),
                ..Default::default()
            }
        }

        pub fn with_channel(mut self, id: &str) -> Self {
            self.channel = Some(id.to_string());
            self
        }

        pub fn by_user(mut self, id: &str) -> Self {
            self.user = Some(id.to_string());
            self
        }

        pub fn time_from(mut self, date: chrono::DateTime<FixedOffset>) -> Self {
            self.ts_from = Some(date.timestamp().to_string());
            self
        }

        pub fn time_to(mut self, date: chrono::DateTime<FixedOffset>) -> Self {
            self.ts_to = Some(date.timestamp().to_string());
            self
        }

        pub fn showing_hidden(mut self) -> Self {
            self.show_files_hidden_by_limit = Some(true);
            self
        }

        pub fn with_images(mut self) -> Self {
            self.insert_type_to_list("images");
            self
        }

        pub fn with_posts(mut self) -> Self {
            self.insert_type_to_list("spaces");
            self
        }

        pub fn with_snippets(mut self) -> Self {
            self.insert_type_to_list("snippets");
            self
        }

        pub fn with_google_docs(mut self) -> Self {
            self.insert_type_to_list("gdocs");
            self
        }

        pub fn with_zips(mut self) -> Self {
            self.insert_type_to_list("zips");
            self
        }

        pub fn with_pdfs(mut self) -> Self {
            self.insert_type_to_list("pdfs");
            self
        }

        fn insert_type_to_list(&mut self, type_to_add: &str) {
            let existing = self
                .types
                .as_mut()
                .map(|types| format!("{types},{type_to_add}"))
                .unwrap_or(String::from(type_to_add));
            self.types = Some(existing);
        }
    }

    impl SlackUrlRequest for FilesListRequest {
        fn get_api_url(&self) -> &str {
            FILES_LIST
        }
    }
}

pub mod response {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{formats::Flexible, TimestampSeconds};

    #[derive(Deserialize, Serialize)]
    pub struct FilesListResponse {
        pub ok: bool,
        pub error: Option<String>,
        pub files: Option<Vec<FilesListItem>>,
    }

    #[serde_with::serde_as]
    #[derive(Deserialize, Serialize)]
    pub struct FilesListItem {
        /// The ID of the file object
        pub id: String,
        /// A Unix timestamp representing when the file was created
        #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
        pub created: Option<DateTime<Utc>>,
        #[serde_as(as = "Option<TimestampSeconds<String, Flexible>>")]
        pub timestamp: Option<DateTime<Utc>>,
        /// Name of the file; may be null for unnamed files
        pub name: Option<String>,
        /// Title of the file
        pub title: String,
        /// The file's mimetype
        pub mimetype: String,
        /// The file's type. Note the mimetype and filetype properties do not have a 1-to-1 mapping,
        /// as multiple different files types ('html', 'js',etc.) share the same mime type
        pub filetype: String,
        /// The ID of the user who created the object
        pub user: String,
        /// Contains the IDs of any channels into which the file is currently shared
        pub channels: Vec<String>,
        /// The filesize in bytes. Snippets are limited to a maximum file size of 1 megabyte
        pub size: usize,
    }
}
