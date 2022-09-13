//! api for block kit https://api.slack.com/reference/block-kit/blocks and using blocks to create messages for slack, etc.

use serde::{Deserialize, Serialize};

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

    pub fn add_section(mut self, block: SectionBlock) -> Self {
        self.blocks.push(BlockType::Section(block));
        self
    }

    pub fn add_section_markdown(mut self, text: &str) -> Self {
        self.blocks
            .push(BlockType::Section(SectionBlock::new_markdown(text)));
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    /// TODO
    Actions,
    Context,
    Divider,
    File(FileBlock),
    Header(HeaderBlock),
    Image(ImageBlock),
    Input,
    Section(SectionBlock),
    Video,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileBlock {
    /// The external unique ID for this file
    pub external_id: String,
    /// At the moment, source will always be remote for a remote file
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageBlock {
    /// The URL of the image to be displayed. Maximum length for this field is 3000 characters
    pub image_url: String,
    /// A plain-text summary of the image. This should not contain any markup. Maximum length for this field is 2000 characters
    pub alt_text: String,
    /// An optional title for the image in the form of a text object that can only be of type: plain_text. Maximum length for the text in this field is 2000 characters.
    pub title: Option<TextObject>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
            .add_section_markdown("header")
            .add_section_markdown("footer");
        let serialized = serde_json::to_string(&builder).unwrap();
        assert_eq!("{\"blocks\":[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"header\"}},{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"footer\"}}]}", serialized.as_str());
    }
}
