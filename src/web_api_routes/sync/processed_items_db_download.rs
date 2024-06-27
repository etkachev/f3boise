use crate::db::queries::processed_items::{get_all_processed_items, ProcessedItem};
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// route for downloading all processed items in db
pub async fn download_processed_items_csv(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_processed_items_csv(&db_pool).await {
        Ok(results) => HttpResponse::Ok().body(results),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn get_processed_items_csv(db: &PgPool) -> Result<Vec<u8>, AppError> {
    let data = get_all_processed_items(db).await?;
    let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);

    for db_item in data.into_iter() {
        if let Err(err) = wrt.serialize(ProcessedCSVItem::from(db_item)) {
            return Err(AppError::from(format!("Error serializing data: {:?}", err)));
        }
    }
    if let Ok(bytes) = wrt.into_inner() {
        Ok(bytes)
    } else {
        Err(AppError::from("Could not parse csv"))
    }
}

impl From<ProcessedItem> for ProcessedCSVItem {
    fn from(value: ProcessedItem) -> Self {
        ProcessedCSVItem {
            item_type: value.item_type.to_string(),
            item_id: value.item_id.to_string(),
            initial_date_processed: value.initial_date_processed.to_string(),
            date_updated: value.date_updated.to_string(),
            amt_processed: value.amt_processed,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedCSVItem {
    pub item_type: String,
    pub item_id: String,
    pub initial_date_processed: String,
    pub date_updated: String,
    pub amt_processed: i32,
}
