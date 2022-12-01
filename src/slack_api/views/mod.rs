//! Api for building and sending views
//! Slack docs: https://api.slack.com/reference/surfaces/views

pub mod request {
    use crate::slack_api::views::payload::ViewPayload;

    pub struct ViewsOpenRequest {
        pub trigger_id: String,
        pub view: ViewPayload,
    }

    impl ViewsOpenRequest {
        pub fn new(trigger_id: &str, view: ViewPayload) -> Self {
            ViewsOpenRequest {
                trigger_id: trigger_id.to_string(),
                view,
            }
        }
    }
}

pub mod payload {
    use crate::slack_api::block_kit::{BlockType, TextObject};
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    #[serde(tag = "type")]
    pub enum ViewPayload {
        Modal(ViewModal),
        // TODO
        Home,
    }

    #[derive(Serialize, Debug)]
    pub struct ViewModal {
        /// The title that appears in the top-left of the modal. Max length of 24 char.
        title: TextObject,
        /// An array of blocks that defines the content of the view. Max of 100 blocks
        blocks: Vec<BlockType>,
        /// An optional plain_text element that defines the text displayed in the close button at the bottom-right of the view.
        /// Max length of 24 characters
        close: Option<TextObject>,
        /// An optional plain_text element that defines the text displayed in the submit button at the bottom-right of the view.
        /// submit is required when an input block is within the blocks array. Max length of 24 characters
        submit: Option<TextObject>,
        /// An optional string that will be sent to your app in view_submission and block_actions events. Max length of 3000 characters
        private_metadata: Option<String>,
        /// An identifier to recognize interactions and submissions of this particular view.
        /// Don't use this to store sensitive information (use private_metadata instead).
        /// Max length of 255 characters
        callback_id: Option<String>,
        /// When set to true, clicking on the close button will clear all views in a modal and close it.
        /// Defaults to false
        clean_on_close: Option<bool>,
        /// Indicates whether Slack will send your request URL a view_closed event when a user clicks the close button.
        /// Defaults to false
        notify_on_close: Option<bool>,
        /// A custom identifier that must be unique for all views on a per-team basis
        external_id: Option<String>,
        /// When set to true, disables the submit button until the user has completed one or more inputs.
        /// This property is for configuration modals.
        submit_disabled: Option<bool>,
    }

    impl ViewModal {
        pub fn is_valid_payload(&self) -> bool {
            self.is_valid_title()
                && self.is_valid_blocks()
                && self.is_valid_close()
                && self.is_valid_submit()
                && self.is_valid_private_meta()
                && self.is_valid_callback_id()
        }

        fn is_valid_callback_id(&self) -> bool {
            if let Some(callback_id) = &self.callback_id {
                callback_id.chars().count() <= 255
            } else {
                true
            }
        }

        fn is_valid_private_meta(&self) -> bool {
            if let Some(private) = &self.private_metadata {
                private.chars().count() <= 3000
            } else {
                true
            }
        }

        fn is_valid_submit(&self) -> bool {
            if let Some(submit) = &self.submit {
                submit.has_le_chars(24)
            } else {
                true
            }
        }

        fn is_valid_close(&self) -> bool {
            if let Some(close) = &self.close {
                close.has_le_chars(24)
            } else {
                true
            }
        }

        fn is_valid_blocks(&self) -> bool {
            self.blocks.len() <= 100
        }

        fn is_valid_title(&self) -> bool {
            self.title.has_le_chars(24)
        }
    }
}
