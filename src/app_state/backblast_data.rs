use super::ao_data::AO;
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
        let has_ao = match self.ao {
            AO::Unknown(_) | AO::DR => false,
            _ => true,
        };

        let has_pax = self.qs.len() > 0 && self.pax.len() > 0;
        let valid_date = self.date > NaiveDate::MIN;
        let valid_event_times = self.event_times.is_some();

        has_ao && has_pax && valid_date && valid_event_times
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
