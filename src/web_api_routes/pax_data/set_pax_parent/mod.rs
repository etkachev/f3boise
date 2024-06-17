use crate::db::pax_parent_tree::{upsert_pax_parent_relationship, ParentPaxRelation};
use crate::shared::common_errors::AppError;
use crate::shared::responses::SuccessResponse;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// new url for settings pax parent relationship
pub async fn set_pax_parent_tree_route(
    db_pool: web::Data<PgPool>,
    req: web::Json<ParentPaxRelation>,
) -> impl Responder {
    match set_pax_parent_tree_entry(&db_pool, &req).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse::ok()),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn set_pax_parent_tree_entry(db: &PgPool, req: &ParentPaxRelation) -> Result<(), AppError> {
    let mut transaction = db.begin().await.expect("Failed to begin transaction");
    upsert_pax_parent_relationship(&mut transaction, req).await?;
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}
