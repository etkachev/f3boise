use crate::shared::constants::{Q_LINE_UP_CANCEL_TEXT, Q_LINE_UP_CLOSED_TEXT};
use crate::slack_api::block_kit::block_elements::OptionElement;
use crate::slack_api::block_kit::{BlockType, SectionBlock};
use crate::web_api_routes::slash_commands::q_line_up::unwrap_message_data;

pub fn get_existing_q_overflow_options() -> Vec<OptionElement> {
    let clear = OptionElement::new(Q_LINE_UP_CANCEL_TEXT, Q_LINE_UP_CANCEL_TEXT);
    let closed = OptionElement::new(Q_LINE_UP_CLOSED_TEXT, Q_LINE_UP_CLOSED_TEXT);
    vec![clear, closed]
}

pub fn get_ao_string_from_blocks<'a>(
    blocks: &'a Option<Vec<BlockType>>,
    ao_combo_str: &'a str,
) -> Option<&'a str> {
    if let Some(blocks) = blocks {
        let (_start, _end, ao) = {
            let text = match blocks.last() {
                Some(BlockType::Section(SectionBlock { text, .. })) => text.text.to_string(),
                _ => String::new(),
            };
            unwrap_message_data(&text)
        };

        match ao {
            None => Some(ao_combo_str),
            Some(_) => None,
        }
    } else {
        None
    }
}
