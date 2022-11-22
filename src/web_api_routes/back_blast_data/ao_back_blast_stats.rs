use crate::app_state::ao_data::AO;
use crate::db::queries::all_back_blasts::back_blasts_by_ao::back_blasts_by_channel_id;
use crate::db::queries::all_back_blasts::calculate_bb_list_stats::get_avg_pax_per_bd;
use crate::db::queries::all_back_blasts::BackBlastJsonData;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct RouteData {
    pub ao_name: String,
}

#[derive(Serialize)]
pub struct AOBackBlastsStats {
    pub back_blasts: Vec<BackBlastJsonData>,
    pub total: usize,
    pub unique_pax: usize,
    pub avg_pax_per_bd: f64,
}

impl AOBackBlastsStats {
    /// new stats for ao back_blasts with some calculations done
    pub fn new(back_blasts: Vec<BackBlastJsonData>) -> Self {
        let total = back_blasts.len();
        let unique_pax = back_blasts
            .iter()
            .fold(HashSet::<String>::new(), |mut acc, bb| {
                for pax in bb.pax.iter() {
                    acc.insert(pax.to_string());
                }
                acc
            })
            .len();

        let avg_pax_per_bd = get_avg_pax_per_bd(&back_blasts);

        AOBackBlastsStats {
            back_blasts,
            total,
            unique_pax,
            avg_pax_per_bd,
        }
    }
}

/// get back blast stats for an ao.
pub async fn get_back_blast_stats_by_ao(
    path: web::Path<RouteData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let ao = AO::from(path.into_inner().ao_name);
    let channel_id = ao.channel_id();

    match back_blasts_by_channel_id(&db_pool, channel_id).await {
        Ok(results) => HttpResponse::Ok().json(AOBackBlastsStats::new(results)),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
