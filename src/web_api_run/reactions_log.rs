use crate::web_api_routes::reactions_log_data::{
    download_full_reactions_log_csv, pre_blast_reaction_data_route,
};
use crate::web_api_routes::sync::db_sync::sync_prod_reactions_log;
use actix_web::{web, Scope};

pub fn service() -> Scope {
    web::scope("/reactions_log")
        .route(
            "/download-csv",
            web::get().to(download_full_reactions_log_csv),
        )
        .route("/sync-db-via-url", web::get().to(sync_prod_reactions_log))
        .route(
            "/pre-blast-data",
            web::get().to(pre_blast_reaction_data_route),
        )
}
