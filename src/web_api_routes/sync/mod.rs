use crate::app_state::MutableAppState;
use crate::db::init::{get_db_users, sync_ao_list};
use crate::migrate_old::save_old_back_blasts;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn sync_old_data(db_pool: web::Data<PgPool>) -> impl Responder {
    if let Err(err) = save_old_back_blasts(&db_pool).await {
        return HttpResponse::BadRequest().body(err.to_string());
    }
    HttpResponse::Ok().finish()
}

pub async fn sync_data(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    app_state: web::Data<MutableAppState>,
) -> impl Responder {
    match sync_ao_list(&db_pool).await {
        Ok(_) => {
            println!("synced ao list");
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    match get_db_users(&db_pool).await {
        Ok(db_users) => {
            let mut app = app_state.app.lock().expect("Could not lock app state");
            app.users.extend(db_users);
            println!("set db users");
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    match web_state.get_users().await {
        Ok(slack_users) => {
            let mut app = app_state.app.lock().expect("Could not lock app state");
            app.users.extend(slack_users.users);
            app.bots = slack_users.bots;
            app.set_self_bot_id();
            println!("set slack users and bots");
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    match web_state.get_public_channels().await {
        Ok(public_channels) => {
            let mut app = app_state.app.lock().expect("Could not lock app state");
            app.channels = public_channels;
            println!("set channels");
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    match app_state.sync_users(&db_pool).await {
        Ok(_) => {
            println!("Synced users");
        }
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    println!("Synced all");

    HttpResponse::Ok().finish()
}
