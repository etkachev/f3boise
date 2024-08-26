use std::fmt::Display;

/// enum of available views (modals) that we generate for the app)
#[derive(Default, PartialEq, Debug)]
pub enum ViewIds {
    PreBlast,
    PreBlastEdit,
    BackBlast,
    BackBlastEdit,
    BlackDiamondRating,
    #[default]
    Unknown,
}

impl Display for ViewIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = uuid::Uuid::new_v4().to_string();
        let (uid, _) = id.split_at(5);
        let str = match self {
            ViewIds::BackBlast => format!("{}::{uid}", BACK_BLAST_ID),
            ViewIds::BackBlastEdit => format!("{}::{uid}", BACK_BLAST_EDIT_ID),
            ViewIds::PreBlast => format!("{}::{uid}", PRE_BLAST_ID),
            ViewIds::PreBlastEdit => format!("{}::{uid}", PRE_BLAST_EDIT_ID),
            ViewIds::BlackDiamondRating => format!("{}::{uid}", BLACK_DIAMOND_RATING_ID),
            ViewIds::Unknown => "UNKNOWN".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for ViewIds {
    fn from(value: &str) -> Self {
        let (value, _) = value.split_once("::").unwrap_or((value, ""));
        match value {
            BACK_BLAST_ID => ViewIds::BackBlast,
            BACK_BLAST_EDIT_ID => ViewIds::BackBlastEdit,
            PRE_BLAST_ID => ViewIds::PreBlast,
            PRE_BLAST_EDIT_ID => ViewIds::PreBlastEdit,
            BLACK_DIAMOND_RATING_ID => ViewIds::BlackDiamondRating,
            _ => ViewIds::Unknown,
        }
    }
}

const BACK_BLAST_ID: &str = "back_blast";
const BACK_BLAST_EDIT_ID: &str = "back_blast_edit";
const PRE_BLAST_ID: &str = "pre_blast";
const PRE_BLAST_EDIT_ID: &str = "pre_blast_edit";
const BLACK_DIAMOND_RATING_ID: &str = "black_diamond_rating";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_blast_conversion_works() {
        let pb = ViewIds::PreBlast.to_string();
        let back = ViewIds::from(pb.as_str());
        assert_eq!(back, ViewIds::PreBlast);
    }

    #[test]
    fn back_blast_conversion() {
        let bb = ViewIds::BackBlast.to_string();
        let back = ViewIds::from(bb.as_str());
        assert_eq!(back, ViewIds::BackBlast);
    }
}
