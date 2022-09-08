use crate::app_state::MutableAppState;
use crate::db::init::{get_db_users, sync_ao_list};
use crate::migrate_old::save_old_back_blasts;
use crate::shared::common_errors::AppError;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

/// sync old backblasts to db.
pub async fn sync_old_data_route(db_pool: web::Data<PgPool>) -> impl Responder {
    if let Err(err) = save_old_back_blasts(&db_pool).await {
        return HttpResponse::BadRequest().body(err.to_string());
    }
    HttpResponse::Ok().finish()
}

/// external call to sync data to state.
pub async fn sync_data_to_state(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    app_state: &MutableAppState,
) -> Result<(), AppError> {
    sync_ao_list(db_pool).await?;
    println!("synced ao list");
    let db_users = get_db_users(db_pool).await?;
    // scoped to limit lock
    {
        let mut app = app_state.app.lock().expect("Could not lock app state");
        app.users.extend(db_users);
    }
    println!("set db users");

    let slack_users = web_state.get_users().await?;
    // scoped to limit lock
    {
        let mut app = app_state.app.lock().expect("Could not lock app state");
        app.users.extend(slack_users.users);
        app.bots = slack_users.bots;
        app.set_self_bot_id();
    }
    println!("set slack users and bots");

    let public_channels = web_state.get_public_channels().await?;
    // scoped to limit lock
    {
        let mut app = app_state.app.lock().expect("Could not lock app state");
        app.channels = public_channels;
    }
    println!("set channels");

    app_state.sync_users(db_pool).await?;
    println!("Synced users");

    println!("Synced all");

    Ok(())
}

/// route for syncing data to state
pub async fn sync_data_route(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    app_state: web::Data<MutableAppState>,
) -> impl Responder {
    match sync_data_to_state(&db_pool, &web_state, &app_state).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
