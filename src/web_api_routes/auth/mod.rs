pub mod internal_auth;

use crate::web_api_state::MutableWebState;
use actix_session::Session;
use actix_web::{cookie::Key, get, http::header, web, HttpResponse, Responder};
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use serde::Deserialize;

pub fn get_key() -> Key {
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
