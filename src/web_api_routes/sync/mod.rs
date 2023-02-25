use crate::app_state::MutableAppState;
use crate::db::init::sync_ao_list;
use crate::db::queries::users::get_db_users;
use crate::migrate_old::{save_old_back_blasts, save_old_q_line_up};
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;

/// sync old backblasts to db.
pub async fn sync_old_data_route(db_pool: web::Data<PgPool>) -> impl Responder {
    if let Err(err) = save_old_back_blasts(&db_pool).await {
        return HttpResponse::BadRequest().body(err.to_string());
    }
    HttpResponse::Ok().finish()
}

/// sync q line up
pub async fn sync_q_line_up(db_pool: web::Data<PgPool>) -> impl Responder {
    if let Err(err) = save_old_q_line_up(&db_pool).await {
        return HttpResponse::BadRequest().body(err.to_string());
    }

    HttpResponse::Ok().body("Finished")
}

/// external call to sync data to state.
pub async fn sync_data_to_state(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    app_state: &MutableAppState,
) -> Result<(), AppError> {
    let mut all_users: HashMap<String, F3User> = HashMap::new();
    let db_users = get_db_users(db_pool).await?;
    all_users.extend(db_users);

    let slack_users = web_state.get_users().await?;
    all_users.extend(slack_users.users);

    // scoped to limit lock
    {
        let mut app = app_state.app.lock().expect("Could not lock app state");
        app.users = all_users;
        app.bots = slack_users.bots;
        app.set_self_bot_id();
    }
    println!("set slack users and bots");

    let public_channels = web_state.get_public_channels().await?;
    sync_ao_list(db_pool).await?;
    println!("synced ao list");
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
        Ok(_) => HttpResponse::Ok().body("Synced!"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
