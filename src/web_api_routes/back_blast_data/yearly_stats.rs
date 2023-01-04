use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::{get_all_within_date_range, BackBlastJsonData};
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

/// yearly stats for whole region
#[derive(Serialize, Debug, Default, PartialEq)]
pub struct YearlyStats {
    /// year these stats are for.
    year: i32,
    /// Average daily attendance across all AOs
    avg_daily_attendance: usize,
    /// Total pax posts.
    total_pax_posts: usize,
    /// Number of weekly post options a PAX has across region.
    weekly_post_options: usize,
    /// number of unique pax posts
    unique_pax_posted: usize,
}

/// stat collection after iterating through back blast data.
#[derive(Debug, Default)]
struct YearlyStatIterCollection {
    year: i32,
    /// total number of pax posts
    pax_posts: usize,
    /// set of all pax (for uniqueness)
    pax: HashSet<String>,
    /// set of all aos collected
    aos: HashSet<AO>,
    /// map of dates and their number of pax posts across aos.
    bb_dates: HashMap<NaiveDate, usize>,
}

impl YearlyStatIterCollection {
    fn new(start: NaiveDate, bb: Vec<BackBlastJsonData>) -> Self {
        let mut result = bb
            .iter()
            .fold(YearlyStatIterCollection::default(), |mut acc, item| {
                let data = BackBlastData::from(item);
                acc.pax_posts += data.total_pax();
                for pax_name in data.get_pax() {
                    acc.pax.insert(pax_name);
                }
                acc.aos.insert(data.ao.clone());
                *acc.bb_dates.entry(data.date).or_insert(0) += data.get_pax().len();
                acc
            });

        result.year = start.year();
        result
    }

    /// get average daily attendance.
    fn get_avg_daily_attendance(&self) -> usize {
        let value_counts = self.bb_dates.values().count();
        // can't divide by zero
        if value_counts == 0 {
            return 0;
        }
        let sum = self.bb_dates.values().sum::<usize>();
        sum / value_counts
    }
}

impl From<YearlyStatIterCollection> for YearlyStats {
    fn from(stats: YearlyStatIterCollection) -> Self {
        let weekly_post_options = stats.aos.iter().fold(0, |mut acc, ao| {
            acc += ao.week_days().len();
            acc
        });

        YearlyStats {
            year: stats.year,
            total_pax_posts: stats.pax_posts,
            unique_pax_posted: stats.pax.len(),
            weekly_post_options,
            avg_daily_attendance: stats.get_avg_daily_attendance(),
        }
    }
}

#[derive(Deserialize)]
pub struct YearlyStatsQuery {
    start: NaiveDate,
    end: NaiveDate,
}

/// route to get yearly stats for whole region
pub async fn get_yearly_stats_route(
    db_pool: web::Data<PgPool>,
    request: web::Query<YearlyStatsQuery>,
) -> impl Responder {
    match get_yearly_stats(&db_pool, request.start, request.end).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// get yearly stats for whole region.
async fn get_yearly_stats(
    db_pool: &PgPool,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<YearlyStats, AppError> {
    if start.year() != end.year() {
        return Err(AppError::General(String::from(
            "Start and end must be the same",
        )));
    }

    let bb = get_all_within_date_range(db_pool, &start, &end).await?;
    let stats_collection = YearlyStatIterCollection::new(start, bb);
    let stats = YearlyStats::from(stats_collection);
    Ok(stats)
}
