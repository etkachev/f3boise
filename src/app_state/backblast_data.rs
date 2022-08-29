use super::ao_data::AO;
use crate::db::db_back_blast::DbBackBlast;
use crate::web_api_routes::slack_events::event_times::EventTimes;
use chrono::NaiveDate;
use std::collections::HashSet;

pub const BACK_BLAST_TAG: &str = "#backblast";

/// General data of a backblast
#[derive(Debug, PartialEq)]
pub struct BackBlastData {
    /// AO this backblast is part of
    pub ao: AO,
    /// list of Q's that led
    pub qs: HashSet<String>,
    /// list of pax that attended workout
    pax: HashSet<String>,
    /// date that workout happened
    pub date: NaiveDate,
    pub event_times: Option<EventTimes>,
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

    pub fn set_pax(&mut self, pax: HashSet<String>) {
        self.pax = pax;
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
}

fn split_comma_string(input: &str) -> HashSet<String> {
    let mut result = HashSet::<String>::new();
    for item in input.split(',').collect::<Vec<&str>>() {
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

impl Default for BackBlastData {
    fn default() -> Self {
        BackBlastData {
            ao: AO::Unknown("EMPTY".to_string()),
            qs: HashSet::new(),
            pax: HashSet::new(),
            date: NaiveDate::MIN,
            event_times: None,
        }
    }
}
