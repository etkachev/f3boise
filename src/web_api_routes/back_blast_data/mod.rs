use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::{get_all, get_all_dd};
use crate::db::queries::missing_back_blasts::get_back_blasts_since;
use crate::web_api_routes::back_blast_data::top_pax_per_ao::get_top_pax_per_ao;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Datelike, Months, NaiveDate, Utc};
use serde::Serialize;
use sqlx::PgPool;
use std::ops::Sub;

pub mod ao_back_blast_stats;
pub mod ao_monthly_leaderboard;
pub mod back_blast_single;
pub mod csv_download_all;
pub mod double_downs;
pub mod pax_leaderboard_graph;
pub mod test_png;
pub mod top_pax_per_ao;
pub mod yearly_stats;

/// route to get all back blast data
pub async fn get_all_back_blasts_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_all(&db_pool).await {
        Ok(list) => {
            let mapped: Vec<BackBlastData> = list.into_iter().map(BackBlastData::from).collect();
            HttpResponse::Ok().json(mapped)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

/// route to get all double down data
pub async fn get_all_double_downs_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_all_dd(&db_pool).await {
        Ok(list) => {
            let mapped: Vec<BackBlastData> = list.into_iter().map(BackBlastData::from).collect();
            HttpResponse::Ok().json(mapped)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

/// route to get double down stats
pub async fn get_double_down_stats_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match double_downs::get_stats(&db_pool).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// get response on top pax per ao
pub async fn get_top_pax_data_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_top_pax_per_ao(&db_pool, None).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[derive(Serialize)]
pub struct MissingBackBlastData {
    ao: String,
    date: NaiveDate,
}

/// get missing back blast data.
pub async fn get_missing_back_blasts(db_pool: web::Data<PgPool>) -> impl Responder {
    let now = Utc::now().date_naive();
    let six_months_ago = now.sub(Months::new(6));
    match get_back_blasts_since(&db_pool, &six_months_ago).await {
        Ok(list) => {
            let mut results: Vec<MissingBackBlastData> = vec![];
            for ao in AO_LIST {
                let mut date_to_check = six_months_ago;

                while date_to_check < now {
                    if ao.week_days().contains(&date_to_check.weekday()) {
                        // if checked date is part of ao week day
                        let exists = list
                            .iter()
                            .any(|item| item.ao == ao.to_string() && item.date == date_to_check);
                        if !exists {
                            results.push(MissingBackBlastData {
                                ao: ao.to_string(),
                                date: date_to_check,
                            });
                        }
                    }
                    date_to_check = date_to_check.succ();
                }
            }

            HttpResponse::Ok().json(results)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
