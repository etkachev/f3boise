use super::ao_data::AO;
use crate::db::db_back_blast::DbBackBlast;
use crate::db::queries::all_back_blasts::{BackBlastFullJsonData, BackBlastJsonData};
use crate::db::save_back_blast::BackBlastDbEntry;
use crate::shared::string_utils::{string_split_hash, string_vec_to_hash};
use crate::web_api_routes::slack_events::event_times::EventTimes;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Display;

pub const BACK_BLAST_TAG: &str = "#backblast";
pub const SLACK_BLAST_TAG: &str = "*slackblast*:";

/// General data of a backblast
#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct BackBlastData {
    /// possible id of backblast if saved in db
    pub id: Option<String>,
    /// AO this backblast is part of
    pub ao: AO,
    /// list of Q's that led
    pub qs: HashSet<String>,
    /// list of pax that attended workout
    pax: HashSet<String>,
    /// date that workout happened
    pub date: NaiveDate,
    /// type of back blast
    pub bb_type: BackBlastType,
    pub event_times: Option<EventTimes>,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    /// explicit list of fngs
    pub fngs: HashSet<String>,
}

impl BackBlastData {
    pub fn new(ao: AO, qs: HashSet<String>, pax: HashSet<String>, date: NaiveDate) -> Self {
        BackBlastData {
            ao,
            qs,
            pax,
            date,
            ..Default::default()
        }
    }

    pub fn with_type(mut self, bb_type: BackBlastType) -> Self {
        self.bb_type = bb_type;
        self
    }

    pub fn set_pax(&mut self, pax: HashSet<String>) {
        self.pax = pax;
    }

    pub fn has_pax(&self) -> bool {
        !self.pax.is_empty()
    }

    pub fn includes_pax(&self, name: &str) -> bool {
        self.get_pax().contains(name)
    }

    /// get all pax (including qs)
    pub fn get_pax(&self) -> HashSet<String> {
        self.pax
            .union(&self.qs)
            .map(|name| name.to_string())
            .collect()
    }

    pub fn total_pax(&self) -> usize {
        let pax = self.get_pax();
        pax.len()
    }

    pub fn set_event_times(&mut self, event_times: EventTimes) {
        self.event_times = Some(event_times);
    }

    pub fn is_valid_back_blast(&self) -> bool {
        let has_ao = !matches!(self.ao, AO::Unknown(_) | AO::DR);

        let has_pax = !self.qs.is_empty() && !self.pax.is_empty();
        let valid_date = self.date > NaiveDate::MIN;
        let valid_event_times = self.event_times.is_some();

        has_ao && has_pax && valid_date && valid_event_times
    }

    /// combo of ao, date, and type
    pub fn get_unique_id(&self) -> String {
        format!("{}-{}-{}", self.ao, self.date, self.bb_type)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Eq, Default, Clone)]
pub enum BackBlastType {
    #[default]
    BackBlast,
    DoubleDown,
    OffTheBooks,
}

impl Display for BackBlastType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BackBlastType::BackBlast => "backblast".to_string(),
            BackBlastType::DoubleDown => "doubledown".to_string(),
            BackBlastType::OffTheBooks => "otb".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for BackBlastType {
    fn from(bb_type: &str) -> Self {
        match bb_type {
            "backblast" => BackBlastType::BackBlast,
            "doubledown" => BackBlastType::DoubleDown,
            "otb" => BackBlastType::OffTheBooks,
            _ => BackBlastType::BackBlast,
        }
    }
}

fn split_comma_string(input: &str) -> HashSet<String> {
    let mut result = HashSet::<String>::new();
    for item in input
        .split(',')
        .filter_map(|name| {
            let name = name.trim();
            if !name.is_empty() {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
    {
        result.insert(item.to_string());
    }
    result
}

impl From<DbBackBlast> for BackBlastData {
    fn from(db_bb: DbBackBlast) -> Self {
        let ao = AO::from(db_bb.ao.clone());
        let qs = split_comma_string(&db_bb.q);
        let pax = split_comma_string(&db_bb.pax);
        BackBlastData::new(ao, qs, pax, db_bb.date)
    }
}

impl From<&BackBlastDbEntry> for BackBlastData {
    fn from(db_entry: &BackBlastDbEntry) -> Self {
        let qs = string_split_hash(&db_entry.q, ',');
        let pax = string_split_hash(&db_entry.pax, ',');
        let fngs = string_split_hash(&db_entry.fngs.clone().unwrap_or_default(), ',');
        BackBlastData {
            id: Some(db_entry.id.to_string()),
            ao: AO::from(db_entry.ao.to_string()),
            qs,
            pax,
            date: db_entry.date,
            bb_type: BackBlastType::from(db_entry.bb_type.as_str()),
            event_times: None,
            fngs,
            title: db_entry.title.clone(),
            moleskine: db_entry.moleskine.clone(),
        }
    }
}

impl From<BackBlastJsonData> for BackBlastData {
    fn from(data: BackBlastJsonData) -> Self {
        BackBlastData::from(&data)
    }
}

impl From<BackBlastFullJsonData> for BackBlastData {
    fn from(value: BackBlastFullJsonData) -> Self {
        BackBlastData::from(&value)
    }
}
impl From<&BackBlastFullJsonData> for BackBlastData {
    fn from(data: &BackBlastFullJsonData) -> Self {
        let qs = string_vec_to_hash(&data.q);
        let pax = string_vec_to_hash(&data.pax);
        let fngs = string_vec_to_hash(&data.fngs.clone().unwrap_or_default());
        BackBlastData {
            id: Some(data.id.to_string()),
            ao: AO::from_channel_id(data.channel_id.as_str()),
            qs,
            pax,
            date: data.date,
            bb_type: BackBlastType::from(data.bb_type.as_str()),
            event_times: None,
            title: data.title.clone(),
            moleskine: data.moleskine.clone(),
            fngs,
        }
    }
}

impl From<&BackBlastJsonData> for BackBlastData {
    fn from(data: &BackBlastJsonData) -> Self {
        let qs = string_vec_to_hash(&data.q);
        let pax = string_vec_to_hash(&data.pax);
        BackBlastData {
            id: Some(data.id.to_string()),
            ao: AO::from_channel_id(data.channel_id.as_str()),
            qs,
            pax,
            date: data.date,
            bb_type: BackBlastType::from(data.bb_type.as_str()),
            event_times: None,
            title: data.title.clone(),
            moleskine: None,
            fngs: HashSet::new(),
        }
    }
}

impl Default for BackBlastData {
    fn default() -> Self {
        BackBlastData {
            id: None,
            ao: AO::Unknown("EMPTY".to_string()),
            qs: HashSet::new(),
            pax: HashSet::new(),
            date: NaiveDate::MIN,
            bb_type: BackBlastType::BackBlast,
            event_times: None,
            title: None,
            moleskine: None,
            fngs: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otb_conversion() {
        let otb = BackBlastType::OffTheBooks.to_string();
        let back = BackBlastType::from(otb.as_str());
        assert_eq!(back, BackBlastType::OffTheBooks);
    }
}
