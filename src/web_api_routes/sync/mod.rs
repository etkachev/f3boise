pub mod db_sync;
mod processed_items_db_download;
mod q_line_up_download_db;

use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::{BackBlastData, BackBlastType};
use crate::app_state::MutableAppState;
use crate::db::init::sync_ao_list;
use crate::db::queries::users::get_db_users;
use crate::migrate_old::{save_old_back_blasts, save_old_q_line_up};
use crate::shared::common_errors::AppError;
use crate::shared::string_utils::string_split_hash;
use crate::users::f3_user::F3User;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::io::Read;

pub use processed_items_db_download::download_processed_items_csv;
pub use q_line_up_download_db::download_q_line_up_csv;

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
        // app.users = all_users;
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

    app_state.sync_users(db_pool, all_users).await?;
    // app_state.sync_users(db_pool).await?;
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

/// extract backblast from csv reader
pub fn extract_back_blasts<R: Read>(
    mut rdr: csv::Reader<R>,
) -> Result<Vec<BackBlastData>, AppError> {
    let mut results: Vec<BackBlastData> = vec![];
    for record in rdr.deserialize() {
        let record: ProdCSVEntry = record?;
        let date = NaiveDate::parse_from_str(record.date.as_str(), "%Y-%m-%d").unwrap();
        let ao = AO::from(record.ao.to_string());
        let qs = string_split_hash(record.q.as_str(), ',');
        let pax = string_split_hash(record.pax.as_str(), ',');
        let mut mapped = BackBlastData::new(ao, qs, pax, date);
        mapped.title.clone_from(&record.title);
        mapped.moleskine.clone_from(&record.moleskine);
        mapped.fngs = string_split_hash(record.fngs.unwrap_or_default().as_str(), ',');
        mapped.bb_type = record
            .bb_type
            .map(|bb_type| BackBlastType::from(bb_type.as_str()))
            .unwrap_or_default();
        results.push(mapped);
    }
    Ok(results)
}

#[derive(Serialize, Deserialize, Debug)]
struct ProdCSVEntry {
    pub ao: String,
    pub q: String,
    pub pax: String,
    pub date: String,
    pub fngs: Option<String>,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    pub bb_type: Option<String>,
}
