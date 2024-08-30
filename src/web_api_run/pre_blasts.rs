use crate::web_api_routes::pre_blast_data;
use crate::web_api_routes::sync::db_sync::sync_prod_pre_blasts;
use actix_web::{web, Scope};

pub fn service() -> Scope {
    web::scope("/pre_blasts")
        .route(
            "/download-csv",
            web::get().to(pre_blast_data::download_pre_blast_data_csv),
        )
        .route("/sync-db-via-url", web::get().to(sync_prod_pre_blasts))
}
