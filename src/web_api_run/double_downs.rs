use crate::web_api_routes::back_blast_data::{
    get_all_double_downs_route, get_double_down_stats_route,
};
use actix_web::{web, Scope};

/// service and routes related to double downs
pub fn service() -> Scope {
    web::scope("/double_downs")
        .route("/all", web::get().to(get_all_double_downs_route))
        .route("/stats", web::get().to(get_double_down_stats_route))
}
