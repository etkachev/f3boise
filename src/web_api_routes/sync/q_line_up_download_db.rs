use crate::db::queries::q_line_up::{get_all_q_line_up_db_items, RawQLineUpDbEntry};
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// api url for downloading q line up as csv
pub async fn download_q_line_up_csv(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_csv_for_q_line_up_db(&db_pool).await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn get_csv_for_q_line_up_db(db: &PgPool) -> Result<Vec<u8>, AppError> {
    let items = get_all_q_line_up_db_items(db).await?;
    let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);

    for db_item in items.into_iter() {
        if let Err(err) = wrt.serialize(QLineUpCSVItem::from(db_item)) {
            return Err(AppError::from(format!("Error serializing data: {:?}", err)));
        }
    }
    if let Ok(bytes) = wrt.into_inner() {
        Ok(bytes)
    } else {
        Err(AppError::from("Could not parse csv"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QLineUpCSVItem {
    pub qs: String,
    pub ao: String,
    pub date: String,
    pub closed: bool,
    pub channel_id: String,
}

impl From<RawQLineUpDbEntry> for QLineUpCSVItem {
    fn from(value: RawQLineUpDbEntry) -> Self {
        QLineUpCSVItem {
            qs: value.qs.to_string(),
            ao: value.ao.to_string(),
            date: value.date.to_string(),
            closed: value.closed,
            channel_id: value.channel_id.to_string(),
        }
    }
}
