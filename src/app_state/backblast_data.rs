use super::ao_data::AO;
use chrono::NaiveDate;

pub const BACK_BLAST_TAG: &str = "#backblast";

/// General data of a backblast
#[derive(Debug, PartialEq)]
pub struct BackBlastData {
    /// AO this backblast is part of
    pub ao: AO,
    /// list of Q's that led
    pub qs: Vec<String>,
    /// list of pax that attended workout
    pax: Vec<String>,
    /// date that workout happened
    pub date: NaiveDate,
}

impl BackBlastData {
    pub fn new(ao: AO, qs: Vec<String>, pax: Vec<String>, date: NaiveDate) -> Self {
        BackBlastData { ao, qs, pax, date }
    }

    pub fn set_pax(&mut self, pax: Vec<String>) {
        self.pax = pax;
    }

    pub fn get_pax(&self) -> Vec<String> {
        let mut pax = self.pax.clone();
        for q in self.qs.iter() {
            if !pax.contains(q) {
                pax.push(q.clone());
            }
        }
        pax
    }

    pub fn total_pax(&self) -> usize {
        let pax = self.get_pax();
        pax.len()
    }
}

impl Default for BackBlastData {
    fn default() -> Self {
        BackBlastData {
            ao: AO::Unknown("EMPTY".to_string()),
            qs: Vec::new(),
            pax: Vec::new(),
            date: NaiveDate::MIN,
        }
    }
}
