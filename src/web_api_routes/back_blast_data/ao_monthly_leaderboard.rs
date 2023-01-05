use crate::web_api_routes::graphs::ao_monthly_leaderboard::get_ao_monthly_stats_graph;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct MonthLeaderboardQuery {
    pub date: Option<NaiveDate>,
}

/// test route for posting monthly ao leaderboard to bot playground channel
pub async fn ao_monthly_leaderboard_route(
    db_pool: web::Data<PgPool>,
    query: web::Query<MonthLeaderboardQuery>,
    web_state: web::Data<MutableWebState>,
) -> impl Responder {
    match get_ao_monthly_stats_graph(
        &db_pool,
        &query.date,
        &web_state,
        String::from("C03TZV5RRF1"),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Saved"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
