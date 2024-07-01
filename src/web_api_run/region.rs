use crate::web_api_routes::back_blast_data::yearly_stats::get_yearly_stats_route;
use crate::web_api_routes::region_data::ao_meta_data::ao_list_meta_data_route;
use actix_web::{web, Scope};

/// service and routes related to region
pub fn service() -> Scope {
    web::scope("/region")
        .route("/workouts", web::get().to(ao_list_meta_data_route))
        .route("/stats", web::get().to(get_yearly_stats_route))
}
