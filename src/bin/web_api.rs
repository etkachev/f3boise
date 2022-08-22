use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key, get, http::header, middleware, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use f3_api_rs::oauth_client::get_oauth_client;
use f3_api_rs::web_api_routes::challenge_event_api::challenge_event_api;
use f3_api_rs::web_api_state::{AppState, LOCAL_URL, PORT_NUMBER, SLACK_SERVER};
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use serde::Deserialize;
use std::env;

fn get_key() -> Key {
    Key::generate()
}

#[get("/")]
async fn index(session: Session) -> impl Responder {
    let access = session.get::<String>("access").unwrap();
    let link = if access.is_some() { "logout" } else { "login" };
    let html = format!(
        r#"
        <html>
        <head><title>Home page</title></head>
        <body>
        <h1>Home</h1>
        <a href="/{}">{}</a>
        </body>
        </html>
        "#,
        link, link,
    );
    HttpResponse::Ok().body(html)
}

#[get("/login")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("channels:read".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.remove("access");

    HttpResponse::Found()
        .append_header((header::LOCATION, "/".to_string()))
        .finish()
}

#[derive(Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

#[get("/auth")]
async fn auth(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());
    let token = &data
        .oauth
        .exchange_code(code)
        .request(http_client)
        .expect("exchange_code failed");
    let access = token.access_token().secret();
    session
        .insert("access", access)
        .expect("Could not set access token to session");
    let html = format!(
        r#"
        <html>
        <head><title>Auth Page</title></head>
        <body>
            <h1>Auth finished</h1>
        </body>
        </html>
        "#
    );
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        dotenv().ok();

        let auth_token = env::var("BOT_OAUTH_TOKEN").expect("No auth token set in env");
        let signing_secret =
            env::var("SLACK_SIGNING_SECRET").expect("No Signing secret set in env");
        let client = get_oauth_client();
        let api_base_url = format!("https://{}/api", SLACK_SERVER);

        println!("Starting on port: {}", PORT_NUMBER);
        App::new()
            .app_data(web::Data::new(AppState {
                api_base_url,
                oauth: client,
                bot_auth_token: auth_token,
                signing_secret,
            }))
            .wrap(middleware::Compress::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                get_key(),
            ))
            .service(index)
            .service(auth)
            .service(login)
            .service(logout)
            .service(challenge_event_api)
    })
    .bind((LOCAL_URL, PORT_NUMBER))?
    .run()
    .await
}
