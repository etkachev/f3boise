use dotenv::dotenv;
use f3_api_rs::configuration::get_configuration;
use f3_api_rs::web_api_run::{init_web_app, run};
use f3_api_rs::web_api_state::LOCAL_URL;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let web_app = init_web_app().await;
    let config = get_configuration().expect("Failed to read config");
    let address = format!("{}:{}", LOCAL_URL, config.application_port);
    let listener = std::net::TcpListener::bind(&address).expect("Could not bind to port");
    run(web_app, listener)?.await
}
