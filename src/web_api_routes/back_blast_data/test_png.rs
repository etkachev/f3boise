use crate::web_api_routes::graphs::overall_pax_leaderboard::post_overall_pax_dd_leaderboard_graph;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn test_png_route(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
) -> impl Responder {
    let bot_playground_channel = String::from("C03TZV5RRF1");
    // %Y/%m/%d-%Y/%m/%d
    let range = "2023/07/09-2023/09/29";
    match post_overall_pax_dd_leaderboard_graph(&db_pool, &web_state, bot_playground_channel, range)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Done"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
