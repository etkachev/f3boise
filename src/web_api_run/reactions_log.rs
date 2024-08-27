use crate::web_api_routes::reactions_log_data::download_full_reactions_log_csv;
use actix_web::{web, Scope};

pub fn service() -> Scope {
    web::scope("/reactions_log").route(
        "/download-csv",
        web::get().to(download_full_reactions_log_csv),
    )
}
