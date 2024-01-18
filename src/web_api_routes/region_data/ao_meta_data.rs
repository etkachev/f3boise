//! generic ao meta data for FE to consume
use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_all_within_date_range;
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::web_api_routes::q_line_up::get_line_up_map;
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use chrono::{Datelike, Duration, NaiveTime};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;

/// route to get work out meta data
pub async fn ao_list_meta_data_route(db: web::Data<PgPool>) -> impl Responder {
    match ao_list_data(&db).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// fetches general ao data along with tomorrow's q and avg pax info.
async fn ao_list_data(db: &PgPool) -> Result<[AoMetaData; 17], AppError> {
    let recent = get_recent_back_blasts(db).await?;
    let now = local_boise_time().date_naive();
    let two_days_later = now + Duration::days(2);
    let tomorrow_qs = get_line_up_map(db, &now, &two_days_later).await?;
    let tomorrow = now + Duration::days(1);
    let tomorrow_week_day = tomorrow.weekday().to_string();
    let results = AO_LIST.map(|ao| {
        let mut meta_data = AoMetaData::from(&ao);
        let filtered_bb = recent
            .iter()
            .filter_map(|bb| {
                if bb.ao == ao {
                    Some(bb.total_pax())
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        let len = filtered_bb.len();
        if len != 0 {
            let sum = filtered_bb.iter().sum::<usize>();
            let avg_pax = (sum as f32) / len as f32;
            let avg_pax = avg_pax.ceil() as usize;
            meta_data.avg_pax_count = avg_pax;
        }

        if meta_data
            .workout_dates
            .get(tomorrow_week_day.as_str())
            .is_some()
        {
            let q = tomorrow_qs
                .get(ao.to_string().as_str())
                .iter()
                .find_map(|q_list| {
                    q_list.iter().find_map(|item| {
                        if item.date == tomorrow {
                            Some(item.qs.join(", "))
                        } else {
                            None
                        }
                    })
                });

            meta_data.tomorrows_q = q;
            meta_data.is_tomorrow = true;
        }

        meta_data
    });
    Ok(results)
}

async fn get_recent_back_blasts(db: &PgPool) -> Result<Vec<BackBlastData>, AppError> {
    let now = local_boise_time().date_naive();
    let three_months = now - Duration::days(90);
    let results = get_all_within_date_range(db, &three_months, &now).await?;
    let data = results
        .iter()
        .map(BackBlastData::from)
        .collect::<Vec<BackBlastData>>();
    Ok(data)
}

#[derive(Serialize)]
pub struct AoMetaData {
    id: String,
    name: String,
    tomorrows_q: Option<String>,
    is_tomorrow: bool,
    address: Option<String>,
    map_location_url: Option<String>,
    avg_pax_count: usize,
    workout_type: String,
    workout_dates: HashMap<String, (NaiveTime, NaiveTime)>,
}

impl From<&AO> for AoMetaData {
    fn from(value: &AO) -> Self {
        AoMetaData {
            id: value.to_string(),
            name: value.friendly_name().to_string(),
            is_tomorrow: false,
            tomorrows_q: None,
            address: value.address().map(|address| address.to_string()),
            map_location_url: value.real_map_url().map(|url| url.to_string()),
            avg_pax_count: 0,
            workout_type: value.ao_type().to_string(),
            workout_dates: value.week_days().iter().fold(
                HashMap::<String, (NaiveTime, NaiveTime)>::new(),
                |mut acc, day| {
                    if let Some(times) = value.start_end_times(day) {
                        acc.insert(day.to_string(), times);
                    }
                    acc
                },
            ),
        }
    }
}
