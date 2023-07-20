use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::get_back_blast_by_id;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SingleRouteData {
    pub id: String,
}

/// return single backblast data
pub async fn get_single_back_blast_data(
    path: web::Path<SingleRouteData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    match get_back_blast_by_id(&db_pool, path.id.as_str()).await {
        Ok(response) => {
            let data = response.map(BackBlastData::from);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
