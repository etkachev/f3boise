//! api for block kit https://api.slack.com/reference/block-kit/blocks and using blocks to create messages for slack, etc.

use crate::slack_api::block_kit::block_elements::{
    BlockElementType, OptionElement, OptionGroupElement,
};
use serde::{Deserialize, Serialize};

pub mod block_elements;

#[derive(Serialize, Debug, Default)]
pub struct BlockBuilder {
    pub blocks: Vec<BlockType>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        BlockBuilder {
            ..Default::default()
        }
    }

    pub fn reached_max(&self) -> bool {
        self.blocks.len() > 40
    }

    pub fn section(mut self, block: SectionBlock) -> Self {
        self.add_section(block);
        self
    }

    pub fn add_section(&mut self, block: SectionBlock) {
        self.blocks.push(BlockType::Section(block));
    }

    pub fn section_markdown(mut self, text: &str) -> Self {
        self.add_section_markdown(text);
        self
    }

    pub fn add_section_markdown(&mut self, text: &str) {
        self.blocks
            .push(BlockType::Section(SectionBlock::new_markdown(text)));
    }

    pub fn img_markdown(mut self, text: &str, img_url: &str, img_alt_text: &str) -> Self {
        self.add_img_markdown(text, img_url, img_alt_text);
        self
    }

    pub fn add_img_markdown(&mut self, text: &str, img_url: &str, img_alt_text: &str) {
        self.blocks
            .push(BlockType::Section(SectionBlock::new_markdown_with_img(
                text,
                img_url,
                img_alt_text,
            )));
    }

    pub fn header(mut self, text: &str) -> Self {
        self.blocks
            .push(BlockType::Header(HeaderBlock::new_plain_text(text)));
        self
    }

    pub fn context(mut self, text: &str) -> Self {
        self.blocks
            .push(BlockType::Context(ContextBlock::new_markdown(text)));
        self
    }

    pub fn divider(mut self) -> Self {
        self.add_divider();
        self
    }

    pub fn add_divider(&mut self) {
        self.blocks.push(BlockType::Divider);
    }

    pub fn plain_input(
        mut self,
        label: &str,
        action_id: &str,
        place_holder: Option<String>,
        initial_value: Option<String>,
        optional: bool,
    ) -> Self {
        self.add_plain_input(label, action_id, place_holder, initial_value, optional);
        self
    }

    pub fn add_plain_input(
        &mut self,
        label: &str,
        action_id: &str,
        place_holder: Option<String>,
        initial_value: Option<String>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_input(label, action_id, place_holder, initial_value).optional(optional),
        ));
    }

    pub fn text_box(
        mut self,
        label: &str,
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
        optional: bool,
    ) -> Self {
        self.add_text_box(label, action_id, placeholder, initial_value, optional);
        self
    }

    pub fn add_text_box(
        &mut self,
        label: &str,
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_text_box(label, action_id, placeholder, initial_value)
                .optional(optional),
        ));
    }

    pub fn select(
        mut self,
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<OptionElement>,
        optional: bool,
    ) -> Self {
        self.add_select(label, action_id, options, default, optional);
        self
    }

    pub fn add_select(
        &mut self,
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<OptionElement>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_select(label, action_id, options, default).optional(optional),
        ));
    }

    pub fn multi_select(
        mut self,
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<Vec<OptionElement>>,
        optional: bool,
    ) -> Self {
        self.add_multi_select(label, action_id, options, default, optional);
        self
    }

    pub fn add_multi_select(
        &mut self,
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<Vec<OptionElement>>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_multi_select(label, action_id, options, default).optional(optional),
        ));
    }

    pub fn channel_select(
        mut self,
        label: &str,
        action_id: &str,
        initial_channel: Option<String>,
        optional: bool,
    ) -> Self {
        self.add_channel_select(label, action_id, initial_channel, optional);
        self
    }

    pub fn add_channel_select(
        &mut self,
        label: &str,
        action_id: &str,
        initial_channel: Option<String>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_channel_select(label, action_id, initial_channel).optional(optional),
        ))
    }

    /// initial date should be in the format YYYY-MM-DD
    pub fn date_picker(
        mut self,
        label: &str,
        action_id: &str,
        date: Option<String>,
        optional: bool,
    ) -> Self {
        self.add_date_picker(label, action_id, date, optional);
        self
    }

    pub fn add_date_picker(
        &mut self,
        label: &str,
        action_id: &str,
        date: Option<String>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_date_picker(label, action_id, date).optional(optional),
        ));
    }

    /// initial time should be in the format HH:mm
    pub fn time_picker(
        mut self,
        label: &str,
        action_id: &str,
        time: Option<String>,
        optional: bool,
    ) -> Self {
        self.add_time_picker(label, action_id, time, optional);
        self
    }

    pub fn add_time_picker(
        &mut self,
        label: &str,
        action_id: &str,
        time: Option<String>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_time_picker(label, action_id, time).optional(optional),
        ));
    }

    pub fn multi_users_select(
        mut self,
        label: &str,
        action_id: &str,
        initial: Option<Vec<String>>,
        optional: bool,
    ) -> Self {
        self.add_multi_users_select(label, action_id, initial, optional);
        self
    }

    pub fn add_multi_users_select(
        &mut self,
        label: &str,
        action_id: &str,
        initial: Option<Vec<String>>,
        optional: bool,
    ) {
        self.blocks.push(BlockType::Input(
            InputBlock::new_multi_users_select(label, action_id, initial).optional(optional),
        ));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Actions(ActionBlock),
    Context(ContextBlock),
    Divider,
    File(FileBlock),
    Header(HeaderBlock),
    Image(ImageBlock),
    Input(InputBlock),
    Section(SectionBlock),
    /// TODO
    Video,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionBlock {
    /// An array of interactive element objects - buttons, select menus, overflow menus, or date pickers. There is a maximum of 25 elements in each action block.
    pub elements: Vec<BlockElementType>,
    /// A string acting as a unique identifier for a block.
    /// If not specified, a block_id will be generated.
    /// You can use this block_id when you receive an interaction payload to identify the source of the action.
    /// Maximum length for this field is 255 characters.
    /// block_id should be unique for each message and each iteration of a message.
    /// If a message is updated, use a new block_id
    pub block_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InputBlock {
    /// A label that appears above an input element in the form of a text object that must have type of plain_text.
    /// Maximum length for the text in this field is 2000 characters.
    pub label: TextObject,
    /// A plain-text input element, a checkbox element, a radio button element,
    /// a select menu element, a multi-select menu element, or a datepicker.
    pub element: BlockElementType,
    /// An optional hint that appears below an input element in a lighter grey.
    /// It must be a text object with a type of plain_text. Maximum length for the text in this field is 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<TextObject>,
    /// A boolean that indicates whether the input element may be empty when a user submits the modal. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl InputBlock {
    pub fn new(label: &str, action_id: &str) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_plain_input(action_id, None, None),
            ..Default::default()
        }
    }

    pub fn new_input(
        label: &str,
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_plain_input(action_id, placeholder, initial_value),
            ..Default::default()
        }
    }

    pub fn new_text_box(
        label: &str,
        action_id: &str,
        placeholder: Option<String>,
        initial_value: Option<String>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_text_box(action_id, placeholder, initial_value),
            ..Default::default()
        }
    }

    pub fn new_select(
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<OptionElement>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_select(action_id, options, default),
            ..Default::default()
        }
    }

    pub fn new_select_with_groups(
        label: &str,
        action_id: &str,
        option_groups: Vec<OptionGroupElement>,
        default: Option<OptionElement>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_select_with_groups(action_id, option_groups, default),
            ..Default::default()
        }
    }

    pub fn new_multi_select(
        label: &str,
        action_id: &str,
        options: Vec<OptionElement>,
        default: Option<Vec<OptionElement>>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_multi_select(action_id, options, default),
            ..Default::default()
        }
    }
    pub fn new_multi_select_with_groups(
        label: &str,
        action_id: &str,
        options: Vec<OptionGroupElement>,
        default: Option<Vec<OptionElement>>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_multi_select_with_groups(action_id, options, default),
            ..Default::default()
        }
    }

    pub fn new_channel_select(
        label: &str,
        action_id: &str,
        initial_channel: Option<String>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_channel_select(action_id, initial_channel),
            ..Default::default()
        }
    }

    pub fn new_date_picker(label: &str, action_id: &str, date: Option<String>) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_date_selector(action_id, date),
            ..Default::default()
        }
    }

    /// initial time should be in the format HH:mm
    pub fn new_time_picker(label: &str, action_id: &str, time: Option<String>) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_time_picker(action_id, time),
            ..Default::default()
        }
    }

    pub fn new_multi_users_select(
        label: &str,
        action_id: &str,
        initial: Option<Vec<String>>,
    ) -> Self {
        InputBlock {
            label: TextObject::new_text(label),
            element: BlockElementType::new_multi_user_select(action_id, initial),
            ..Default::default()
        }
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileBlock {
    /// The external unique ID for this file
    pub external_id: String,
    /// At the moment, source will always be remote for a remote file
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SectionBlock {
    /// The text for the block, in the form of a text object.
    /// Maximum length for the text in this field is 3000 characters.
    /// This field is not required if a valid array of fields objects is provided instead
    pub text: TextObject,
    /// Required if no text is provided. An array of text objects.
    /// Any text objects included with fields will be rendered in a compact format that allows for 2 columns of side-by-side text.
    /// Maximum number of items is 10. Maximum length for the text in each item is 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TextObject>>,
    /// One of the available element objects: https://api.slack.com/reference/messaging/block-elements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<BlockElementType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl SectionBlock {
    pub fn new(text: TextObject) -> Self {
        SectionBlock {
            text,
            ..Default::default()
        }
    }

    pub fn new_markdown(text: &str) -> Self {
        SectionBlock {
            text: TextObject::new_markdown(text),
            ..Default::default()
        }
    }

    pub fn new_plain_text(text: &str) -> Self {
        SectionBlock {
            text: TextObject::new_text(text),
            ..Default::default()
        }
    }

    pub fn new_markdown_with_btn(text: &str, btn_text: &str, action_id: &str) -> Self {
        SectionBlock {
            text: TextObject::new_markdown(text),
            accessory: Some(BlockElementType::new_btn(btn_text, action_id)),
            ..Default::default()
        }
    }

    pub fn new_markdown_with_danger_btn(text: &str, btn_text: &str, action_id: &str) -> Self {
        SectionBlock {
            text: TextObject::new_markdown(text),
            accessory: Some(BlockElementType::new_danger_btn(btn_text, action_id)),
            ..Default::default()
        }
    }

    pub fn new_markdown_with_img(text: &str, image_url: &str, img_alt_text: &str) -> Self {
        SectionBlock {
            text: TextObject::new_markdown(text),
            accessory: Some(BlockElementType::new_img(image_url, img_alt_text)),
            ..Default::default()
        }
    }

    pub fn new_markdown_with_overflow(
        text: &str,
        action_id: &str,
        options: Vec<OptionElement>,
    ) -> Self {
        SectionBlock {
            text: TextObject::new_markdown(text),
            accessory: Some(BlockElementType::new_overflow(action_id, options)),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContextBlock {
    /// An array of image elements and text objects. Maximum number of items is 10. TODO add image support
    elements: Vec<TextObject>,
}

impl ContextBlock {
    pub fn new_text(text: &str) -> Self {
        ContextBlock {
            elements: vec![TextObject::new_text(text)],
        }
    }

    pub fn new_markdown(text: &str) -> Self {
        ContextBlock {
            elements: vec![TextObject::new_markdown(text)],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderBlock {
    /// The text for the block, in the form of a plain_text text object. Maximum length for the text in this field is 150 characters
    text: TextObject,
}

impl HeaderBlock {
    pub fn new(text: TextObject) -> Self {
        HeaderBlock { text }
    }

    pub fn new_markdown(text: &str) -> Self {
        HeaderBlock {
            text: TextObject::new_markdown(text),
        }
    }

    pub fn new_plain_text(text: &str) -> Self {
        HeaderBlock {
            text: TextObject::new_text(text),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageBlock {
    /// The URL of the image to be displayed. Maximum length for this field is 3000 characters
    pub image_url: String,
    /// A plain-text summary of the image. This should not contain any markup. Maximum length for this field is 2000 characters
    pub alt_text: String,
    /// An optional title for the image in the form of a text object that can only be of type: plain_text. Maximum length for the text in this field is 2000 characters.
    pub title: Option<TextObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextObject {
    #[serde(rename = "type", flatten)]
    pub text_type: TextType,
    /// The text for the block. This field accepts any of the standard formatting for mrkdwn. Max length is 3000 char.
    pub text: String,
    /// Indicates whether emojis in a text field should be escaped into the colon emoji format. This field is only usable when type is plain_text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
    /// When set to false (as is default) URLs will be auto-converted into links, conversation names will be link-ified, and certain mentions will be automatically parsed.
    /// Using a value of true will skip any preprocessing of this nature, although you can still include manual parsing strings. This field is only usable when type is mrkdwn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
}

impl TextObject {
    pub fn new_text(text: &str) -> Self {
        TextObject {
            text_type: TextType::PlainText,
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn new_markdown(text: &str) -> Self {
        TextObject {
            text_type: TextType::Mrkdwn,
            text: text.to_string(),
            ..Default::default()
        }
    }

    /// has less than or equal to character amount
    pub fn has_le_chars(&self, char_count: usize) -> bool {
        self.text.chars().count() <= char_count
    }
}

impl Default for TextObject {
    fn default() -> Self {
        TextObject {
            text_type: TextType::PlainText,
            text: String::new(),
            emoji: None,
            verbatim: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum TextType {
    /// Plain text
    PlainText,
    /// Markdown
    Mrkdwn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_correctly() {
        let builder = BlockBuilder::new()
            .section_markdown("header")
            .section_markdown("footer");
        let serialized = serde_json::to_string(&builder).unwrap();
        assert_eq!("{\"blocks\":[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"header\"}},{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"footer\"}}]}", serialized.as_str());
    }
}
