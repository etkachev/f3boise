use crate::slack_api::block_kit::TextObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockElementType {
    Button(ButtonElement),
    Checkboxes,
    Datepicker,
    Image,
    MultiStaticSelect,
    MultiExternalSelect,
    MultiUsersSelect,
    MultiConversationsSelect,
    MultiChannelsSelect,
    Overflow,
    PlainTextInput,
    RadioButtons,
    StaticSelect,
    ExternalSelect,
    UsersSelect,
    ConversationsSelect,
    ChannelsSelect,
    Timepicker,
}

impl BlockElementType {
    pub fn new_btn(text: &str, action_id: &str) -> Self {
        BlockElementType::Button(ButtonElement::new(text, action_id))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonElement {
    /// can only be plain_text
    text: TextObject,
    /// An identifier for this action. You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters
    action_id: String,
}

impl ButtonElement {
    pub fn new(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
        }
    }
}
