use dotenv::dotenv;
// use f3_api_rs::app_state;
// use std::env;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    // let auth_token = env::var("BOT_OAUTH_TOKEN").expect("No auth token set in env");

    // let mut app = app_state::AppState::new(auth_token);

    // app.initialize_data().await;

    // app.get_back_blasts().await;
}
