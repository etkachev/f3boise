use crate::web_api_routes::q_line_up::q_line_up_route;
use crate::web_api_routes::sync::db_sync::sync_q_line_up_db;
use crate::web_api_routes::sync::download_q_line_up_csv;
use actix_web::{web, Scope};

/// service and route related to q line up
pub fn service() -> Scope {
    web::scope("/q_line_up")
        .route("/list", web::get().to(q_line_up_route))
        .route("/download-csv", web::get().to(download_q_line_up_csv))
        .route("/sync-items-via-url", web::get().to(sync_q_line_up_db))
}
