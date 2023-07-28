use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::double_downs::DoubleDownProgram;
use crate::db::queries::all_back_blasts::{get_all_dd_within_date_range, BackBlastJsonData};
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct DoubleDownStats {
    pub current_program: String,
    pub pax: Vec<DoubleDownPaxInfo>,
}

#[derive(Serialize)]
pub struct DoubleDownPaxInfo {
    pub name: String,
    pub post_count: usize,
    pub q_count: usize,
}

impl DoubleDownPaxInfo {
    pub fn new_with_post(name: &str) -> Self {
        DoubleDownPaxInfo {
            name: name.to_string(),
            post_count: 1,
            q_count: 0,
        }
    }
}

impl DoubleDownStats {
    pub fn new(program: &DoubleDownProgram) -> Self {
        DoubleDownStats {
            current_program: program.to_string(),
            pax: vec![],
        }
    }

    pub fn with_data(mut self, data: &[BackBlastJsonData]) -> Self {
        let items = data.iter().map(BackBlastData::from).fold(
            HashMap::<String, DoubleDownPaxInfo>::new(),
            |mut acc, dd| {
                for pax in dd.get_pax() {
                    let pax = pax.to_lowercase();
                    acc.entry(pax.to_string())
                        .and_modify(|entry| {
                            entry.post_count += 1;
                        })
                        .or_insert(DoubleDownPaxInfo::new_with_post(pax.as_str()));
                }

                for qs in dd.qs {
                    let q = qs.to_lowercase();
                    // no need to insert since logic above already included pax.
                    acc.entry(q).and_modify(|entry| entry.q_count += 1);
                }

                acc
            },
        );

        let mut list =
            items
                .into_iter()
                .fold(Vec::<DoubleDownPaxInfo>::new(), |mut acc, (_, info)| {
                    acc.push(info);
                    acc
                });

        list.sort_by(|a, b| b.post_count.cmp(&a.post_count));
        self.pax = list;
        self
    }
}

/// get double down leaderboard and stats
pub async fn get_stats(db_pool: &PgPool) -> Result<DoubleDownStats, AppError> {
    let now = local_boise_time().date_naive();
    let program = DoubleDownProgram::from(&now);
    let date_range = program.date_range();
    let dd_data = get_all_dd_within_date_range(db_pool, &date_range.start, &date_range.end).await?;
    let result = DoubleDownStats::new(&program).with_data(&dd_data);
    Ok(result)
}
