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

pub fn default_post_option(channel_id: Option<&str>) -> OptionElement {
    let blast_where = channel_id
        .map(|id| BlastWhere::CurrentChannel(id.to_string()))
        .unwrap_or_default();
    OptionElement::from(blast_where)
}

pub fn where_to_post_list(channel_id: &str) -> Vec<OptionElement> {
    vec![
        OptionElement::from(BlastWhere::default()),
        OptionElement::from(BlastWhere::CurrentChannel(channel_id.to_string())),
        OptionElement::from(BlastWhere::Myself),
    ]
}

/// default back_blast type for modal
pub fn default_back_blast_type(back_blast_type: Option<BackBlastType>) -> OptionElement {
    OptionElement::from(back_blast_type.unwrap_or_default())
}

/// get list of back blast types for modal
pub fn back_blast_types_list() -> Vec<OptionElement> {
    vec![
        default_back_blast_type(None),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_post_where() {
        let option = default_post_option(Some("321"));
        assert_eq!(option.text.text, "Current Channel".to_string());
        assert_eq!(option.value, "current_channel::321".to_string());
    }

    #[test]
    fn default_back_blast_type_check() {
        let option = default_back_blast_type(Some(BackBlastType::DoubleDown));
        assert_eq!(option.text.text, "Double Down".to_string());
        assert_eq!(option.value, "doubledown".to_string());
    }
}
