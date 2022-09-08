use crate::app_state::MutableAppState;
use crate::configuration::{DatabaseSettings, Settings};
use crate::db::DbStore;
use crate::oauth_client::get_oauth_client;
use crate::shared::common_errors::AppError;
use crate::web_api_routes::auth::get_key;
use crate::web_api_routes::back_blast_data::get_all_back_blasts_route;
use crate::web_api_routes::pax_data::{get_pax_info, get_users};
use crate::web_api_routes::slack_events::slack_events;
use crate::web_api_routes::sync::{sync_data_route, sync_old_data_route};
use crate::web_api_state::{MutableWebState, SLACK_SERVER};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, AppError> {
        let connection_pool = get_connection_pool(&configuration.database);

        // TODO update
        let web_state = init_web_state();
        let app_state = MutableAppState::new();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(web_state, app_state, listener, connection_pool)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

fn init_web_state() -> MutableWebState {
    let auth_token = env::var("BOT_OAUTH_TOKEN").expect("No auth token set in env");
    let signing_secret = env::var("SLACK_SIGNING_SECRET").expect("No Signing secret set in env");
    let verify_token = env::var("DEPRECATED_VERIFY_TOKEN").expect("No Verify token set in env");
    let client = get_oauth_client();
    let base_api_url = format!("https://{}/api/", SLACK_SERVER);
    MutableWebState {
        token: auth_token.to_string(),
        base_api_url,
        oauth: client,
        bot_auth_token: auth_token,
        signing_secret,
        verify_token,
        db: DbStore::new(),
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("F3 Boise")
}

pub fn run(
    web_app: MutableWebState,
    app_state: MutableAppState,
    tcp_listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let web_app_data = web::Data::new(web_app);
    let db_pool = web::Data::new(db_pool);
    let app_state_data = web::Data::new(app_state);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                get_key(),
            ))
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/events", web::post().to(slack_events))
            .route("/sync", web::get().to(sync_data_route))
            .route("/sync-old", web::get().to(sync_old_data_route))
            .service(
                web::scope("/pax")
                    .route("/info", web::get().to(get_pax_info))
                    .route("/all", web::get().to(get_users)),
            )
            .service(
                web::scope("/back_blasts").route("/all", web::get().to(get_all_back_blasts_route)),
            )
            .app_data(web_app_data.clone())
            .app_data(app_state_data.clone())
            .app_data(db_pool.clone())
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
