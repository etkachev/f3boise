use super::ao_data::AO;
use crate::db::db_back_blast::DbBackBlast;
use crate::web_api_routes::slack_events::event_times::EventTimes;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const BACK_BLAST_TAG: &str = "#backblast";
pub const SLACK_BLAST_TAG: &str = "*slackblast*:";

/// General data of a backblast
#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct BackBlastData {
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

        if !has_ao {
            println!("No ao: {:?}", self.ao);
        }

        if !has_pax {
            println!(
                "No pax - qs: {:?}, pax: {:?}, ALL: {:?}",
                self.qs,
                self.pax,
                self.get_pax()
            );
        }

        if !valid_date {
            println!("invalid date: {:?}", self.date);
        }

        if !valid_event_times {
            println!("Ts times invalid: {:?}", self.event_times);
        }

        has_ao && has_pax && valid_date && valid_event_times
    }

    /// combo of ao, date, and type
    pub fn get_unique_id(&self) -> String {
        format!(
            "{}-{}-{}",
            self.ao.to_string(),
            self.date,
            self.bb_type.to_string()
        )
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Eq)]
pub enum BackBlastType {
    BackBlast,
    // TODO more types like DoubleDown, etc
}

impl ToString for BackBlastType {
    fn to_string(&self) -> String {
        match self {
            BackBlastType::BackBlast => "backblast".to_string(),
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

impl Default for BackBlastData {
    fn default() -> Self {
        BackBlastData {
            ao: AO::Unknown("EMPTY".to_string()),
            qs: HashSet::new(),
            pax: HashSet::new(),
            date: NaiveDate::MIN,
            bb_type: BackBlastType::BackBlast,
            event_times: None,
        }
    }
}
