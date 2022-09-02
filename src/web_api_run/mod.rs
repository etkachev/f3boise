use crate::db::DbStore;
use crate::oauth_client::get_oauth_client;
use crate::web_api_routes::pax_data::get_pax_info;
use crate::web_api_routes::slack_events::slack_events;
use crate::web_api_state::{MutableWebState, SLACK_SERVER};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::dev::Server;
use actix_web::{
    cookie::Key, get, http::header, middleware, web, App, HttpResponse, HttpServer, Responder,
};
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use serde::Deserialize;
use std::env;
use std::sync::Mutex;

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
async fn login(data: web::Data<MutableWebState>) -> impl Responder {
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
    data: web::Data<MutableWebState>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    println!("state is :{}", params.state);
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
    let html = r#"
        <html>
        <head><title>Auth Page</title></head>
        <body>
            <h1>Auth finished</h1>
        </body>
        </html>
        "#
    .to_string();
    HttpResponse::Ok().body(html)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn init_web_app() -> MutableWebState {
    let auth_token = env::var("BOT_OAUTH_TOKEN").expect("No auth token set in env");
    let signing_secret = env::var("SLACK_SIGNING_SECRET").expect("No Signing secret set in env");
    let verify_token = env::var("DEPRECATED_VERIFY_TOKEN").expect("No Verify token set in env");
    let client = get_oauth_client();
    let base_api_url = format!("https://{}/api/", SLACK_SERVER);
    let data_app = crate::app_state::AppState::new();

    let mut web_app = MutableWebState {
        token: auth_token.to_string(),
        base_api_url,
        oauth: client,
        bot_auth_token: auth_token,
        signing_secret,
        verify_token,
        app: Mutex::new(data_app),
        db: DbStore::new(),
    };

    web_app.initialize_data().await;

    web_app
}

pub fn run(
    web_app: MutableWebState,
    tcp_listener: std::net::TcpListener,
) -> Result<Server, std::io::Error> {
    let web_app_data = web::Data::new(web_app);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web_app_data.clone())
            .wrap(middleware::Compress::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                get_key(),
            ))
            .service(index)
            .route("/health_check", web::get().to(health_check))
            .service(auth)
            .service(login)
            .service(logout)
            .service(slack_events)
            .service(web::scope("/pax").service(get_pax_info))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
