use crate::slack_api::block_kit::TextObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    Overflow(OverflowElement),
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

    pub fn new_danger_btn(text: &str, action_id: &str) -> Self {
        BlockElementType::Button(ButtonElement::new_danger(text, action_id))
    }

    pub fn new_overflow(action_id: &str, options: Vec<OptionElement>) -> Self {
        BlockElementType::Overflow(OverflowElement::new(action_id, options))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverflowElement {
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block.
    /// Maximum length for this field is 255 characters
    action_id: String,
    /// An array of up to five option objects to display in the menu
    options: Vec<OptionElement>,
}

impl OverflowElement {
    pub fn new(action_id: &str, options: Vec<OptionElement>) -> Self {
        OverflowElement {
            action_id: action_id.to_string(),
            options,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionElement {
    /// A text object that defines the text shown in the option on the menu.
    /// Overflow, select, and multi-select menus can only use plain_text objects,
    /// while radio buttons and checkboxes can use mrkdwn text objects.
    /// Maximum length for the text in this field is 75 characters
    pub text: TextObject,
    /// A unique string value that will be passed to your app when this option is chosen.
    /// Maximum length for this field is 75 characters
    pub value: String,
}

impl OptionElement {
    pub fn new(text: &str, value: &str) -> Self {
        OptionElement {
            text: TextObject::new_text(text),
            value: value.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonElement {
    /// can only be plain_text
    text: TextObject,
    /// An identifier for this action. You can use this when you receive an interaction payload to identify the source of the action.
    /// Should be unique among all other action_ids in the containing block. Maximum length for this field is 255 characters
    action_id: String,
    /// either 'primary' or 'danger'. uses default if none passed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

pub enum ButtonStyle {
    Primary,
    Danger,
}

const BTN_PRIMARY_STYLE: &str = "primary";
const BTN_DANGER_STYLE: &str = "danger";

impl ToString for ButtonStyle {
    fn to_string(&self) -> String {
        match self {
            ButtonStyle::Danger => BTN_DANGER_STYLE.to_string(),
            ButtonStyle::Primary => BTN_PRIMARY_STYLE.to_string(),
        }
    }
}

impl ButtonElement {
    /// new button with default style
    pub fn new(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: None,
        }
    }

    /// new button with danger style
    pub fn new_danger(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: Some(ButtonStyle::Danger.to_string()),
        }
    }

    /// new button with primary style
    pub fn new_primary(text: &str, action_id: &str) -> Self {
        ButtonElement {
            text: TextObject::new_text(text),
            action_id: action_id.to_string(),
            style: Some(ButtonStyle::Primary.to_string()),
        }
    }
}
