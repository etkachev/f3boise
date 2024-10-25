use crate::web_api_routes::back_blast_data::{
    get_all_double_downs_route, get_combined_double_downs_route, get_double_down_stats_route,
    get_general_double_down_info_route,
};
use actix_web::{web, Scope};

/// service and routes related to double downs
pub fn service() -> Scope {
    web::scope("/double_downs")
        .route("/all", web::get().to(get_all_double_downs_route))
        .route("/stats", web::get().to(get_double_down_stats_route))
        .route("/info", web::get().to(get_general_double_down_info_route))
        .route("/combined", web::get().to(get_combined_double_downs_route))
}
