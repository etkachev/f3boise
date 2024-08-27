use crate::db::queries::reactions_log;
use crate::db::save_reaction_log::ReactionLogDbItem;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ReactionLogRow {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub reaction: String,
    pub slack_user: String,
    pub reaction_added: bool,
    pub reaction_timestamp: NaiveDateTime,
}

impl From<ReactionLogDbItem> for ReactionLogRow {
    fn from(value: ReactionLogDbItem) -> Self {
        ReactionLogRow {
            id: value.id.to_string(),
            entity_type: value.entity_type.to_string(),
            entity_id: value.entity_id.to_string(),
            reaction: value.reaction.to_string(),
            slack_user: value.slack_user.to_string(),
            reaction_added: value.reaction_added,
            reaction_timestamp: value.reaction_timestamp,
        }
    }
}

/// route to download full reactions log for csv
pub async fn download_full_reactions_log_csv(db: web::Data<PgPool>) -> impl Responder {
    match reactions_log::get_full_reaction_log(&db).await {
        Ok(results) => {
            let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
            for item in results.into_iter() {
                if let Err(err) = wrt.serialize(ReactionLogRow::from(item)) {
                    println!("error serializing {:?}", err);
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
