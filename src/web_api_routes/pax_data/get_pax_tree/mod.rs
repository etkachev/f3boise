use crate::db::queries::users::get_pax_tree_relationship;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// get pax tree list
pub async fn get_pax_tree(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_pax_tree_relationship(&db_pool).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
