use crate::db::save_back_blast;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::sync::extract_back_blasts;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use std::io::Cursor;

#[derive(Deserialize)]
pub struct SyncProdReq {
    url: String,
}

pub async fn sync_prod_back_blasts(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_back_blasts(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_back_blasts(url: &str, db: &PgPool) -> Result<(), AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let reader = Cursor::new(bytes);
    let rdr = csv::ReaderBuilder::new().from_reader(reader);
    let results = extract_back_blasts(rdr)?;
    save_back_blast::save_multiple(db, &results).await?;
    Ok(())
}
