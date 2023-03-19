/// enum of available views (modals) that we generate for the app)
#[derive(Default, PartialEq, Debug)]
pub enum ViewIds {
    PreBlast,
    BackBlast,
    #[default]
    Unknown,
}

impl ToString for ViewIds {
    fn to_string(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let (uid, _) = id.split_at(5);
        match self {
            ViewIds::BackBlast => format!("{}::{uid}", BACK_BLAST_ID),
            ViewIds::PreBlast => format!("{}::{uid}", PRE_BLAST_ID),
            ViewIds::Unknown => "UNKNOWN".to_string(),
        }
    }
}

impl From<&str> for ViewIds {
    fn from(value: &str) -> Self {
        let (value, _) = value.split_once("::").unwrap_or((value, ""));
        match value {
            BACK_BLAST_ID => ViewIds::BackBlast,
            PRE_BLAST_ID => ViewIds::PreBlast,
            _ => ViewIds::Unknown,
        }
    }
}

const BACK_BLAST_ID: &str = "back_blast";
const PRE_BLAST_ID: &str = "pre_blast";

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
