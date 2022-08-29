use crate::app_state::backblast_data::BackBlastData;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbBackBlast {
    #[serde(rename = "AO")]
    pub ao: String,
    #[serde(rename = "Q")]
    pub q: String,
    #[serde(rename = "PAX")]
    pub pax: String,
    #[serde(rename = "DATE")]
    pub date: NaiveDate,
}

impl From<&BackBlastData> for DbBackBlast {
    fn from(data: &BackBlastData) -> Self {
        let mut q: Vec<String> = data.qs.clone().into_iter().collect();
        let mut pax: Vec<String> = data.get_pax().into_iter().collect();
        q.sort();
        pax.sort();
        DbBackBlast {
            ao: data.ao.to_string(),
            q: q.join(","),
            pax: pax.join(","),
            date: data.date,
        }
    }
}

impl DbBackBlast {
    /// generate unique id based on ao and date combined.
    pub fn get_unique_id(&self) -> String {
        format!("{}-{}", self.ao, self.date)
    }
}
