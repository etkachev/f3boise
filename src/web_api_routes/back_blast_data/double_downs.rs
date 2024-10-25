use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::double_downs::{DoubleDownProgram, PROGRAM_LIST};
use crate::db::queries::all_back_blasts::{
    get_all_dd, get_all_dd_within_date_range, BackBlastJsonData,
};
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
pub struct GeneralDoubleDownInfo {
    pub current_program: String,
    pub all_programs: Vec<DoubleDownProgram>,
}

impl GeneralDoubleDownInfo {
    pub fn new(program: &DoubleDownProgram) -> Self {
        Self {
            current_program: program.to_string(),
            all_programs: PROGRAM_LIST.to_vec(),
        }
    }
}

#[derive(Serialize)]
pub struct CombinedDoubleDownStats {
    pub data: HashMap<String, Vec<DoubleDownPaxInfo>>,
}

impl CombinedDoubleDownStats {
    pub fn new(all_data: &[BackBlastJsonData]) -> Self {
        let data = PROGRAM_LIST.iter().fold(HashMap::new(), |mut acc, item| {
            let dd_stats = DoubleDownStats::new(item, all_data);
            acc.insert(dd_stats.current_program.to_string(), dd_stats.pax);
            acc
        });
        CombinedDoubleDownStats { data }
    }
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
    pub fn new(program: &DoubleDownProgram, data: &[BackBlastJsonData]) -> Self {
        let items = data
            .iter()
            .filter(|item| program.date_range().contains(&item.date))
            .map(BackBlastData::from)
            .fold(
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

        DoubleDownStats {
            current_program: program.to_string(),
            pax: list,
        }
    }
}

/// get double down leaderboard and stats
pub async fn get_stats(db_pool: &PgPool) -> Result<DoubleDownStats, AppError> {
    let now = local_boise_time().date_naive();
    let program = DoubleDownProgram::from(&now);
    let date_range = program.date_range();
    let dd_data = get_all_dd_within_date_range(db_pool, &date_range.start, &date_range.end).await?;
    let result = DoubleDownStats::new(&program, &dd_data);
    Ok(result)
}

/// get full list of possible double down programs and current one.
pub async fn get_general_info() -> Result<GeneralDoubleDownInfo, AppError> {
    let now = local_boise_time().date_naive();
    let program = DoubleDownProgram::from(&now);
    Ok(GeneralDoubleDownInfo::new(&program))
}

/// get combined dd stats with all programs
pub async fn get_combined_stats(db: &PgPool) -> Result<CombinedDoubleDownStats, AppError> {
    let dd_data = get_all_dd(db).await?;
    let result = CombinedDoubleDownStats::new(&dd_data);
    Ok(result)
}
