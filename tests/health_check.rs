use dotenv::dotenv;
use f3_api_rs::app_state::MutableAppState;
use f3_api_rs::configuration::get_configuration;
use f3_api_rs::web_api_run::{get_connection_pool, run};
use f3_api_rs::web_api_state::{MutableWebState, LOCAL_URL};
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

fn spawn_app() -> String {
    dotenv().ok();
    let default_state = MutableWebState::default();
    let app_state = MutableAppState::new();
    let address = format!("{}:{}", LOCAL_URL, 0);
    let listener = TcpListener::bind(&address).expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let config = get_configuration().expect("Failed to get config");
    let pg_pool = get_connection_pool(&config.database);
    let server =
        run(default_state, app_state, listener, pg_pool).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", LOCAL_URL, port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    println!("running on address: {}", address);
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn users_returns_a_200() {
    // arrange
    let _address = spawn_app();
    let configuration = get_configuration().expect("failed to read config");
    let connection_string = configuration.database.connection_string();
    // The `Connection` trait must be in scope for us to invoke
    let mut connection = PgConnection::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    // act
    let saved = sqlx::query!("SELECT name FROM users")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved user");
    assert_eq!(saved.name, "Stinger");
}
