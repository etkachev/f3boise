use crate::app_state::ao_data::AO;
use crate::web_api_routes::graphs::ao_pax_leaderboard::post_ao_pax_leaderboard_graph;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct PaxLeaderboardQuery {
    pub ao: String,
}

/// route to post pax leaderboard
pub async fn pax_leaderboard_route(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    query: web::Query<PaxLeaderboardQuery>,
) -> impl Responder {
    let ao = AO::from(query.ao.to_string());
    match post_ao_pax_leaderboard_graph(&db_pool, &web_state, ao.channel_id().to_string()).await {
        Ok(_) => HttpResponse::Ok().body("Done"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
