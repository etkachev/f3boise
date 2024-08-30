use crate::db::queries::pre_blasts;
use crate::db::queries::pre_blasts::PreBlastJsonFullData;
use actix_web::{web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreBlastRow {
    pub id: String,
    pub ao: String,
    pub title: String,
    /// comma separated
    pub qs: String,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub why: String,
    /// comma separated
    pub equipment: String,
    pub fng_message: String,
    pub mole_skin: String,
    /// comma separated
    pub img_ids: String,
    pub ts: String,
}

impl From<PreBlastJsonFullData> for PreBlastRow {
    fn from(value: PreBlastJsonFullData) -> Self {
        PreBlastRow {
            id: value.id.to_string(),
            ao: value.ao.to_string(),
            title: value.title.to_string(),
            qs: value.qs.join(","),
            date: value.date,
            start_time: value.start_time,
            why: value.why,
            equipment: value.equipment.map(|e| e.join(",")).unwrap_or_default(),
            fng_message: value.fng_message.unwrap_or_default(),
            mole_skin: value.mole_skin.unwrap_or_default(),
            img_ids: value
                .img_ids
                .map(|img_ids| img_ids.join(","))
                .unwrap_or_default(),
            ts: value.ts.unwrap_or_default(),
        }
    }
}

/// route to download the preblast data in csv format
pub async fn download_pre_blast_data_csv(db_pool: web::Data<PgPool>) -> impl Responder {
    match pre_blasts::get_all_pre_blasts(&db_pool).await {
        Ok(results) => {
            let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
            for pb in results.into_iter() {
                if let Err(err) = wrt.serialize(PreBlastRow::from(pb)) {
                    println!("Error serializing data: {:?}", err);
                    return HttpResponse::BadRequest().body(err.to_string());
                }
            }
            if let Ok(bytes) = wrt.into_inner() {
                HttpResponse::Ok().body(bytes)
            } else {
                HttpResponse::BadRequest().body("Could not parse csv")
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
