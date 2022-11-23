use crate::db::queries::all_back_blasts::{get_list_with_pax, BackBlastJsonData};
use crate::db::queries::users::get_user_by_name;
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use crate::web_api_routes::pax_data::PaxInfoResponse;
use crate::web_api_routes::slash_commands::my_stats::get_pax_info_from_bb_data;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RouteData {
    /// name of pax to get stats for
    name: String,
}

#[derive(Serialize)]
pub struct PaxStatsResponse {
    pub q_count: usize,
    pub post_count: usize,
    pub first_post: NaiveDate,
    pub pax_profile: Option<F3User>,
    /// full list of BD's for this pax
    pub bd_list: Vec<BackBlastJsonData>,
}

impl PaxStatsResponse {
    /// build stats response from combo of data
    pub fn new(
        user: Option<F3User>,
        pax_info: PaxInfoResponse,
        bd_list: Vec<BackBlastJsonData>,
    ) -> Self {
        PaxStatsResponse {
            q_count: pax_info.q_count,
            post_count: pax_info.post_count,
            first_post: pax_info.start_date,
            pax_profile: user,
            bd_list,
        }
    }
}

/// route for getting some stats on a certain pax
pub async fn pax_stats_route(
    db_pool: web::Data<PgPool>,
    route_data: web::Path<RouteData>,
) -> impl Responder {
    match get_pax_stats(&db_pool, route_data.into_inner().name.as_str()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// get the stats for certain pax
async fn get_pax_stats(db_pool: &PgPool, name: &str) -> Result<PaxStatsResponse, AppError> {
    let list = get_list_with_pax(db_pool, name).await?;
    let response = get_pax_info_from_bb_data(&list, name);
    let user = get_user_by_name(db_pool, name).await?;
    let stats = PaxStatsResponse::new(user, response, list);
    Ok(stats)
}
