use crate::app_state::ao_data::AO;
use chrono::NaiveDate;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum InteractionTypes {
    QLineUp(QSheetActionComboData),
    EditBackBlast(String),
    Unknown,
}

impl InteractionTypes {
    pub fn new_q_line_up(date: &NaiveDate, ao: &AO) -> Self {
        InteractionTypes::QLineUp(QSheetActionComboData::new_q_line_up(date, ao))
    }

    /// pass in the id of saved backblast for editing later
    pub fn new_edit_back_blast(id: &str) -> Self {
        InteractionTypes::EditBackBlast(id.to_string())
    }
}

impl From<&str> for InteractionTypes {
    fn from(action_id: &str) -> Self {
        let (first_type, rest) = action_id.split_once("::").unwrap_or((action_id, ""));
        match first_type {
            Q_LINE_UP => InteractionTypes::QLineUp(QSheetActionComboData::from(rest)),
            EDIT_BACK_BLAST => InteractionTypes::EditBackBlast(rest.to_string()),
            _ => InteractionTypes::Unknown,
        }
    }
}

impl Display for InteractionTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            InteractionTypes::QLineUp(data) => format!("{Q_LINE_UP}::{}", data),
            InteractionTypes::EditBackBlast(id) => format!("{EDIT_BACK_BLAST}::{}", id),
            InteractionTypes::Unknown => "unknown".to_string(),
        };
        write!(f, "{}", str)
    }
}

const Q_LINE_UP: &str = "q_line_up";
const EDIT_BACK_BLAST: &str = "edit_back_blast";

#[derive(Debug, PartialEq)]
pub struct QSheetActionComboData {
    pub date: NaiveDate,
    pub ao: AO,
}

impl QSheetActionComboData {
    fn new_q_line_up(date: &NaiveDate, ao: &AO) -> Self {
        QSheetActionComboData {
            date: *date,
            ao: ao.clone(),
        }
    }
}

impl From<&str> for QSheetActionComboData {
    fn from(action_id: &str) -> Self {
        let (date, ao) = action_id.split_once("::").unwrap_or((action_id, ""));

        let date = NaiveDate::from_str(date).unwrap_or(NaiveDate::MIN);
        if date == NaiveDate::MIN {
            return QSheetActionComboData::default();
        }

        let ao = AO::from(ao.to_string());
        QSheetActionComboData { date, ao }
    }
}

impl Display for QSheetActionComboData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_args!("{}::{}", self.date, self.ao))
    }
}

impl Default for QSheetActionComboData {
    fn default() -> Self {
        QSheetActionComboData {
            date: NaiveDate::MIN,
            ao: AO::Unknown(String::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_sheet_action_convert() {
        let action_combo = InteractionTypes::from("q_line_up::2022-03-01::bleach");
        assert_eq!(
            action_combo,
            InteractionTypes::QLineUp(QSheetActionComboData {
                ao: AO::Bleach,
                date: NaiveDate::from_ymd_opt(2022, 3, 1).unwrap()
            })
        );
    }
}
