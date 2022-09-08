use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_all;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

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
