use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_all_within_date_range;
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::BlockBuilder;
use chrono::{Months, NaiveDate};
use serde::Serialize;
use sqlx::PgPool;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;

/// get some stats around top pax per ao and overall
pub async fn get_top_pax_per_ao(
    db_pool: &PgPool,
    dates: Option<(NaiveDate, NaiveDate)>,
) -> Result<BlockBuilder, AppError> {
    let default_end_date = local_boise_time().date_naive();
    let default_start_date = default_end_date.sub(Months::new(1));
    let (start_date, end_date) = dates.unwrap_or((default_start_date, default_end_date));
    let bb_list = get_all_within_date_range(db_pool, &start_date, &end_date).await?;
    let mut result = AllAOStats::new();

    for bb in bb_list.into_iter().map(BackBlastData::from) {
        result.for_ao_q(&bb.ao, &bb.qs);
        result.for_ao_pax(&bb.ao, &bb.get_pax());
    }

    let mut builder = BlockBuilder::new()
        .header("Top PAX for all AOs")
        .context(format!("{} - {}", start_date, end_date).as_str());

    for (ao, stats) in result.aos {
        builder.add_section_markdown(format!("*{}*", ao.to_string()).as_str());
        let (top_qs, amount) = stats.top_qs();
        builder.add_section_markdown(format!("Top Q posts: {:?} - {}", top_qs, amount).as_str());
        let (top_pax, amount) = stats.top_pax();
        builder.add_section_markdown(format!("Top Pax Posts: {:?} - {}", top_pax, amount).as_str());
        builder.add_divider();
    }

    Ok(builder)
}

#[derive(Serialize, Debug, Default)]
pub struct TopPaxStats {
    /// map of each pax name and total amount of q posts for this AO
    pub qs_count: HashMap<String, u16>,
    /// map of each pax name and total amount of HC posts for this AO
    pub pax_count: HashMap<String, u16>,
}

impl TopPaxStats {
    pub fn new() -> TopPaxStats {
        TopPaxStats {
            qs_count: HashMap::new(),
            pax_count: HashMap::new(),
        }
    }

    pub fn for_q(&mut self, pax: &str) {
        if let Some(existing) = self.qs_count.get_mut(pax) {
            *existing += 1;
        } else {
            self.qs_count.insert(pax.to_string(), 1);
        }
    }

    pub fn for_pax(&mut self, pax: &str) {
        if let Some(existing) = self.pax_count.get_mut(pax) {
            *existing += 1;
        } else {
            self.pax_count.insert(pax.to_string(), 1);
        }
    }

    pub fn top_qs(&self) -> (Vec<String>, u16) {
        self.top_results(&self.qs_count)
    }

    pub fn top_pax(&self) -> (Vec<String>, u16) {
        self.top_results(&self.pax_count)
    }

    fn top_results(&self, list: &HashMap<String, u16>) -> (Vec<String>, u16) {
        let mut top_qs: (Vec<String>, u16) = (Vec::new(), 0);

        for (q, count) in list {
            match count.cmp(&top_qs.1) {
                Ordering::Greater => {
                    top_qs.0 = vec![q.to_string()];
                    top_qs.1 = *count;
                }
                Ordering::Equal => top_qs.0.push(q.to_string()),
                _ => (),
            }
        }
        top_qs
    }
}

#[derive(Serialize, Debug, Default)]
pub struct AllAOStats {
    pub aos: HashMap<AO, TopPaxStats>,
    pub overall: TopPaxStats,
}

impl AllAOStats {
    pub fn new() -> Self {
        AllAOStats {
            aos: HashMap::new(),
            overall: TopPaxStats::new(),
        }
    }

    pub fn for_ao_q(&mut self, ao: &AO, pax: &HashSet<String>) {
        if let Some(ao_stats) = self.aos.get_mut(ao) {
            for pax_item in pax.iter() {
                ao_stats.for_q(pax_item);
            }
        } else {
            for pax_item in pax.iter() {
                let mut new_stats = TopPaxStats::new();
                new_stats.for_q(pax_item);
                self.aos.insert(ao.clone(), new_stats);
            }
        }
    }

    pub fn for_ao_pax(&mut self, ao: &AO, pax: &HashSet<String>) {
        if let Some(ao_stats) = self.aos.get_mut(ao) {
            for pax_item in pax.iter() {
                ao_stats.for_pax(pax_item);
            }
        } else {
            for pax_item in pax.iter() {
                let mut new_stats = TopPaxStats::new();
                new_stats.for_pax(pax_item);
                self.aos.insert(ao.clone(), new_stats);
            }
        }
    }
}
