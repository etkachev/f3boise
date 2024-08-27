use crate::web_api_routes::pre_blast_data;
use actix_web::{web, Scope};

pub fn service() -> Scope {
    web::scope("/pre_blasts").route(
        "/download-csv",
        web::get().to(pre_blast_data::download_pre_blast_data_csv),
    )
}
