use crate::web_api_routes::back_blast_data::ao_back_blast_stats::get_back_blast_stats_by_ao;
use crate::web_api_routes::back_blast_data::ao_monthly_leaderboard::ao_monthly_leaderboard_route;
use crate::web_api_routes::back_blast_data::back_blast_single::get_single_back_blast_data;
use crate::web_api_routes::back_blast_data::csv_download_all::{
    back_blasts_csv_html, download_back_blasts_csv_route,
};
use crate::web_api_routes::back_blast_data::pax_leaderboard_graph::pax_leaderboard_route;
use crate::web_api_routes::back_blast_data::remind_missing_back_blasts::remind_missing_back_blasts;
use crate::web_api_routes::back_blast_data::{
    get_all_back_blasts_route, get_missing_back_blasts, get_top_pax_data_route,
};
use crate::web_api_routes::sync::db_sync::sync_prod_back_blasts;
use actix_web::{web, Scope};

/// service and routes related to back blasts
pub fn service() -> Scope {
    web::scope("/back_blasts")
        .route("/all", web::get().to(get_all_back_blasts_route))
        .route("/missing", web::get().to(get_missing_back_blasts))
        .route("/top-pax", web::get().to(get_top_pax_data_route))
        .route(
            "/remind-missing-bb",
            web::get().to(remind_missing_back_blasts),
        )
        .route(
            "/monthly-leaderboard",
            web::get().to(ao_monthly_leaderboard_route),
        )
        .route(
            "/pax-leaderboard-graph",
            web::get().to(pax_leaderboard_route),
        )
        // .route("/test-png", web::get().to(test_png_route))
        .route("/download", web::get().to(back_blasts_csv_html))
        .route(
            "/download-csv",
            web::get().to(download_back_blasts_csv_route),
        )
        .route("/sync-via-url", web::get().to(sync_prod_back_blasts))
        .route("/single/{id}", web::get().to(get_single_back_blast_data))
        .route("/{ao_name}", web::get().to(get_back_blast_stats_by_ao))
}
