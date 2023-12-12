use f3_api_rs::configuration::get_configuration;
use f3_api_rs::migrate_old::{cleanup_pax_in_channels, sync_prod_db};
use f3_api_rs::web_api_run::get_connection_pool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PaxCount {
    pub name: String,
    #[serde(rename = "Post Count")]
    pub post_count: u16,
}

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read config");
    let connection_pool = get_connection_pool(&config.database);
    if let Err(err) = sync_prod_db(&connection_pool).await {
        println!("Error syncing prod db to local: {:?}", err);
    }

    // if let Err(err) = cleanup_pax_in_channels(&connection_pool).await {
    //     println!("Error cleaning up PAX: {:?}", err);
    // }
}
