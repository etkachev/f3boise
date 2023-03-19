use crate::app_state::backblast_data::BackBlastType;
use crate::shared::common_errors::AppError;
use crate::slack_api::block_kit::block_elements::OptionElement;
use std::str::FromStr;

pub mod value_utils;
pub mod view_ids;

/// where to post a modal submission response. (for preblast or backblast)
#[derive(Debug, Default)]
pub enum BlastWhere {
    #[default]
    AoChannel,
    CurrentChannel(String),
    Myself,
}

impl ToString for BlastWhere {
    fn to_string(&self) -> String {
        match self {
            BlastWhere::AoChannel => String::from("Ao Channel"),
            BlastWhere::CurrentChannel(_) => String::from("Current Channel"),
            BlastWhere::Myself => String::from("Me"),
        }
    }
}

impl FromStr for BlastWhere {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let split_text = text.split_once("::").unwrap_or((text, ""));
        match split_text {
            ("ao_channel", _) => Ok(BlastWhere::AoChannel),
            ("current_channel", id) => Ok(BlastWhere::CurrentChannel(id.to_string())),
            ("self", _) => Ok(BlastWhere::Myself),
            _ => Err(AppError::General("Could not parse".to_string())),
        }
    }
}

impl From<BlastWhere> for OptionElement {
    fn from(value: BlastWhere) -> Self {
        match value {
            BlastWhere::AoChannel => {
                OptionElement::new(&BlastWhere::AoChannel.to_string(), "ao_channel")
            }
            BlastWhere::CurrentChannel(channel_id) => OptionElement::new(
                &BlastWhere::CurrentChannel(channel_id.to_string()).to_string(),
                &format!("current_channel::{channel_id}"),
            ),
            BlastWhere::Myself => OptionElement::new(&BlastWhere::Myself.to_string(), "self"),
        }
    }
}

pub fn default_post_option() -> OptionElement {
    OptionElement::from(BlastWhere::default())
}

pub fn where_to_post_list(channel_id: &str) -> Vec<OptionElement> {
    vec![
        default_post_option(),
        OptionElement::from(BlastWhere::CurrentChannel(channel_id.to_string())),
        OptionElement::from(BlastWhere::Myself),
    ]
}

/// default back_blast type for modal
pub fn default_back_blast_type() -> OptionElement {
    OptionElement::from(BackBlastType::BackBlast)
}

/// get list of back blast types for modal
pub fn back_blast_types_list() -> Vec<OptionElement> {
    vec![
        default_back_blast_type(),
        OptionElement::from(BackBlastType::DoubleDown),
        OptionElement::from(BackBlastType::OffTheBooks),
    ]
}

impl From<BackBlastType> for OptionElement {
    fn from(value: BackBlastType) -> Self {
        match value {
            BackBlastType::BackBlast => OptionElement::new("BD", value.to_string().as_str()),
            BackBlastType::DoubleDown => {
                OptionElement::new("Double Down", value.to_string().as_str())
            }
            BackBlastType::OffTheBooks => {
                OptionElement::new("Off the Books", value.to_string().as_str())
            }
        }
    }
}
