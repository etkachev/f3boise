use crate::app_state::ao_data::AO;
use chrono::NaiveDate;
use std::str::FromStr;

pub enum InteractionTypes {
    QLineUp,
    Unknown,
}

impl From<&str> for InteractionTypes {
    fn from(text: &str) -> Self {
        match text {
            Q_LINE_UP => InteractionTypes::QLineUp,
            _ => InteractionTypes::Unknown,
        }
    }
}

impl ToString for InteractionTypes {
    fn to_string(&self) -> String {
        let value = match self {
            InteractionTypes::QLineUp => Q_LINE_UP,
            InteractionTypes::Unknown => "unknown",
        };

        value.to_string()
    }
}

const Q_LINE_UP: &str = "q_line_up";

pub struct ActionComboData {
    pub interaction_type: InteractionTypes,
    pub date: NaiveDate,
    pub ao: AO,
}

impl ActionComboData {
    pub fn new_q_line_up(date: &NaiveDate, ao: &AO) -> Self {
        ActionComboData {
            interaction_type: InteractionTypes::QLineUp,
            date: *date,
            ao: ao.clone(),
        }
    }
}

impl From<&str> for ActionComboData {
    fn from(action_id: &str) -> Self {
        let split_action: Vec<&str> = action_id.splitn(3, "::").collect();
        if split_action.len() != 3 {
            return ActionComboData::default();
        }
        let interaction_type = split_action[0];
        let interaction_type = InteractionTypes::from(interaction_type);
        let date = split_action[1];
        let date = NaiveDate::from_str(date).unwrap_or(NaiveDate::MIN);
        if date == NaiveDate::MIN {
            return ActionComboData::default();
        }

        let ao = split_action[2];
        let ao = AO::from(ao.to_string());
        ActionComboData {
            interaction_type,
            date,
            ao,
        }
    }
}

impl ToString for ActionComboData {
    fn to_string(&self) -> String {
        format!(
            "{}::{}::{}",
            self.interaction_type.to_string(),
            self.date,
            self.ao.to_string()
        )
    }
}

impl Default for ActionComboData {
    fn default() -> Self {
        ActionComboData {
            interaction_type: InteractionTypes::Unknown,
            date: NaiveDate::MIN,
            ao: AO::Unknown(String::new()),
        }
    }
}
