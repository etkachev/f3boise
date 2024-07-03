use crate::db::pax_parent_tree;
use crate::db::pax_parent_tree::{F3Parent, ParentPaxRelation};
use crate::db::queries::processed_items;
use crate::db::save_back_blast;
use crate::db::save_q_line_up;
use crate::db::save_user::{sync_user, DbUser};
use crate::shared::common_errors::AppError;
use crate::web_api_routes::pax_data::get_pax_tree::ParentPaxCSVItem;
use crate::web_api_routes::sync::extract_back_blasts;
use crate::web_api_routes::sync::processed_items_db_download::ProcessedCSVItem;
use crate::web_api_routes::sync::q_line_up_download_db::QLineUpCSVItem;
use crate::web_api_routes::sync::users_db_download::UserCSVItem;
use actix_web::web::Bytes;
use actix_web::{web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use csv::Reader;
use serde::Deserialize;
use sqlx::PgPool;
use std::io::{Cursor, Read};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SyncProdReq {
    url: String,
}

pub async fn sync_prod_back_blasts(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_back_blasts(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_back_blasts(url: &str, db: &PgPool) -> Result<(), AppError> {
    let rdr = get_data_bytes_to_reader(url).await?;
    let results = extract_back_blasts(rdr)?;
    save_back_blast::sync_multiple(db, &results).await?;
    Ok(())
}

/// sync prod table for pax_parents_relationships
pub async fn sync_prod_pax_parents(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_pax_parents(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_pax_parents(url: &str, db: &PgPool) -> Result<(), AppError> {
    let rdr = get_data_bytes_to_reader(url).await?;
    let results = extract_pax_parents(rdr)?;
    pax_parent_tree::upsert_multiple_pax_parent_relationships(db, &results).await?;
    Ok(())
}

fn extract_pax_parents<R: Read>(mut rdr: Reader<R>) -> Result<Vec<ParentPaxRelation>, AppError> {
    let mut results: Vec<ParentPaxRelation> = vec![];
    for record in rdr.deserialize() {
        let record: ParentPaxCSVItem = record?;
        let relation = ParentPaxRelation::try_from(record)?;
        results.push(relation);
    }
    Ok(results)
}

/// sync prod table for pax_parents_relationships
pub async fn sync_processed_items(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_process_items(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_process_items(url: &str, db: &PgPool) -> Result<(), AppError> {
    let rdr = get_data_bytes_to_reader(url).await?;
    let results = extract_processed_items(rdr)?;
    processed_items::sync_db_processed_items(db, &results).await?;
    Ok(())
}

fn extract_processed_items<R: Read>(mut rdr: Reader<R>) -> Result<Vec<(String, String)>, AppError> {
    let mut results: Vec<(String, String)> = vec![];

    for record in rdr.deserialize() {
        let record: ProcessedCSVItem = record?;
        results.push((record.item_type.to_string(), record.item_id.to_string()));
    }

    Ok(results)
}

/// route to sync db items for q line up table
pub async fn sync_q_line_up_db(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_q_line_up(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_q_line_up(url: &str, db: &PgPool) -> Result<(), AppError> {
    let rdr = get_data_bytes_to_reader(url).await?;
    let results = extract_q_line_up(rdr)?;
    save_q_line_up::save_list(db, &results).await?;
    Ok(())
}

fn extract_q_line_up<R: Read>(
    mut rdr: Reader<R>,
) -> Result<Vec<save_q_line_up::NewQLineUpDbEntry>, AppError> {
    let mut results: Vec<save_q_line_up::NewQLineUpDbEntry> = vec![];

    for record in rdr.deserialize() {
        let record: QLineUpCSVItem = record?;
        results.push(save_q_line_up::NewQLineUpDbEntry::from(record));
    }

    Ok(results)
}

impl From<QLineUpCSVItem> for save_q_line_up::NewQLineUpDbEntry {
    fn from(value: QLineUpCSVItem) -> Self {
        save_q_line_up::NewQLineUpDbEntry {
            qs: value.qs.to_string(),
            ao: value.ao.to_string(),
            channel_id: value.channel_id.to_string(),
            date: NaiveDate::parse_from_str(&value.date, "%Y-%m-%d").unwrap(),
            closed: value.closed,
            id: Uuid::new_v4(),
        }
    }
}

/// route to sync users from deployed url
pub async fn sync_users_db(
    db_pool: web::Data<PgPool>,
    req: web::Query<SyncProdReq>,
) -> impl Responder {
    match fetch_and_sync_users_db(&req.url, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_and_sync_users_db(url: &str, db: &PgPool) -> Result<(), AppError> {
    let rdr = get_data_bytes_to_reader(url).await?;
    let results = extract_users_from_csv(rdr)?;
    let mut transaction = db.begin().await.expect("Failed to begin transaction");
    for user in results.iter() {
        sync_user(&mut transaction, user).await?;
    }
    transaction
        .commit()
        .await
        .expect("Could not commit transaction");
    Ok(())
}

fn extract_users_from_csv<R: Read>(mut rdr: Reader<R>) -> Result<Vec<DbUser>, AppError> {
    let mut results: Vec<DbUser> = vec![];

    for record in rdr.deserialize() {
        let record: UserCSVItem = record?;
        results.push(DbUser::from(record));
    }

    Ok(results)
}

impl From<UserCSVItem> for DbUser {
    fn from(value: UserCSVItem) -> Self {
        DbUser {
            name: value.name.to_string(),
            email: value.email.to_string(),
            slack_id: value.slack_id.to_string(),
            img_url: value.img_url.clone(),
            parent: None,
            create_date: NaiveDateTime::parse_from_str(&value.create_date, "%Y-%m-%d %H:%M:%S%.f")
                .unwrap_or_default(),
        }
    }
}

async fn get_data_bytes_to_reader(url: &str) -> Result<Reader<Cursor<Bytes>>, AppError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let reader = Cursor::new(bytes);
    Ok(csv::ReaderBuilder::new().from_reader(reader))
}

impl TryFrom<ParentPaxCSVItem> for ParentPaxRelation {
    type Error = AppError;

    fn try_from(value: ParentPaxCSVItem) -> Result<Self, Self::Error> {
        let parent = serde_json::from_str::<F3Parent>(value.parent.as_str())?;
        Ok(ParentPaxRelation {
            pax_name: value.pax_name.to_string(),
            slack_id: value.slack_id.clone(),
            parent,
        })
    }
}
