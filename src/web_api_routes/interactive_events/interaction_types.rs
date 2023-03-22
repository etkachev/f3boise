use crate::app_state::ao_data::AO;
use chrono::NaiveDate;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum InteractionTypes {
    QLineUp(QSheetActionComboData),
    EditBackBlast,
    Unknown,
}

impl InteractionTypes {
    pub fn new_q_line_up(date: &NaiveDate, ao: &AO) -> Self {
        InteractionTypes::QLineUp(QSheetActionComboData::new_q_line_up(date, ao))
    }

    pub fn new_edit_back_blast() -> Self {
        InteractionTypes::EditBackBlast
    }
}

impl From<&str> for InteractionTypes {
    fn from(action_id: &str) -> Self {
        let (first_type, rest) = action_id.split_once("::").unwrap_or((action_id, ""));
        match first_type {
            Q_LINE_UP => InteractionTypes::QLineUp(QSheetActionComboData::from(rest)),
            EDIT_BACK_BLAST => InteractionTypes::EditBackBlast,
            _ => InteractionTypes::Unknown,
        }
    }
}

impl ToString for InteractionTypes {
    fn to_string(&self) -> String {
        match self {
            InteractionTypes::QLineUp(data) => format!("{Q_LINE_UP}::{}", data.to_string()),
            InteractionTypes::EditBackBlast => EDIT_BACK_BLAST.to_string(),
            InteractionTypes::Unknown => "unknown".to_string(),
        }
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

impl ToString for QSheetActionComboData {
    fn to_string(&self) -> String {
        format!("{}::{}", self.date, self.ao.to_string())
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
                date: NaiveDate::from_ymd(2022, 3, 1)
            })
        );
    }
}
