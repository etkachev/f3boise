/// enum of available views (modals) that we generate for the app)
#[derive(Default)]
pub enum ViewIds {
    PreBlast,
    BackBlast,
    #[default]
    Unknown,
}

impl ToString for ViewIds {
    fn to_string(&self) -> String {
        match self {
            ViewIds::BackBlast => BACK_BLAST_ID,
            ViewIds::PreBlast => PRE_BLAST_ID,
            ViewIds::Unknown => "UNKNOWN",
        }
        .to_string()
    }
}

impl From<&str> for ViewIds {
    fn from(value: &str) -> Self {
        match value {
            BACK_BLAST_ID => ViewIds::BackBlast,
            PRE_BLAST_ID => ViewIds::PreBlast,
            _ => ViewIds::Unknown,
        }
    }
}

const BACK_BLAST_ID: &str = "back_blast";
const PRE_BLAST_ID: &str = "pre_blast";
