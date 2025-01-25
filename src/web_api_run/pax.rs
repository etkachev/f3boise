use crate::web_api_routes::pax_data::direct_message::{
    send_direct_message_route, test_welcome_direct_message,
};
use crate::web_api_routes::pax_data::get_pax_tree::{
    download_pax_relationship_csv_route, get_pax_tree,
};
use crate::web_api_routes::pax_data::pax_leaderboards::post_pax_leaderboards;
use crate::web_api_routes::pax_data::set_pax_parent::set_pax_parent_tree_route;
use crate::web_api_routes::pax_data::stats::pax_stats_route;
use crate::web_api_routes::pax_data::{
    get_bad_data, get_pax_back_blasts, get_pax_double_downs, get_pax_info, get_users,
};
use crate::web_api_routes::sync::db_sync::{sync_prod_pax_parents, sync_users_db};
use crate::web_api_routes::sync::users_db_csv_download;
use actix_web::{web, Scope};

/// service and routes related to pax
pub fn service() -> Scope {
    web::scope("/pax")
        .route("/info", web::get().to(get_pax_info))
        .route("/back_blasts", web::get().to(get_pax_back_blasts))
        .route("/double_downs", web::get().to(get_pax_double_downs))
        .route("/all", web::get().to(get_users))
        .route("/bad-data", web::get().to(get_bad_data))
        .route("/post-leaderboard", web::get().to(post_pax_leaderboards))
        .route("/download_users_csv", web::get().to(users_db_csv_download))
        .route("/sync-users-via-url", web::get().to(sync_users_db))
        .route("/set-pax-parent", web::post().to(set_pax_parent_tree_route))
        .route("/tree", web::get().to(get_pax_tree))
        .route("/dm", web::post().to(send_direct_message_route))
        .route(
            "/test-welcome-dm",
            web::post().to(test_welcome_direct_message),
        )
        .route(
            "/sync-pax-parent-via-url",
            web::get().to(sync_prod_pax_parents),
        )
        .route(
            "/download-parent-pax-csv",
            web::get().to(download_pax_relationship_csv_route),
        )
        .route("/stats/{name}", web::get().to(pax_stats_route))
}
