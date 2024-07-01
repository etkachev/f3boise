use crate::web_api_routes::sync::db_sync::sync_processed_items;
use crate::web_api_routes::sync::download_processed_items_csv;
use actix_web::{web, Scope};

/// service and routes related to processed items
pub fn service() -> Scope {
    web::scope("/processed_items")
        .route("/download_csv", web::get().to(download_processed_items_csv))
        .route("/sync-items-via-url", web::get().to(sync_processed_items))
}
