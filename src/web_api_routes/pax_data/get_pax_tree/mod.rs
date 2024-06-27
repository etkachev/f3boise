use crate::db::queries::users::{
    get_pax_parent_relationship_entries, get_pax_tree_relationship, ParentPaxRelationDbItem,
};
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// get pax tree list
pub async fn get_pax_tree(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_pax_tree_relationship(&db_pool).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// url for downloading csv for pax parent relationship
pub async fn download_pax_relationship_csv_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_pax_relationship_data(&db_pool).await {
        Ok(bytes) => HttpResponse::Ok().body(bytes),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn get_pax_relationship_data(db: &PgPool) -> Result<Vec<u8>, AppError> {
    let data = get_pax_parent_relationship_entries(db).await?;

    let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
    for db_item in data.into_iter() {
        if let Err(err) = wrt.serialize(ParentPaxCSVItem::from(db_item)) {
            return Err(AppError::from(format!("Error serializing data: {:?}", err)));
        }
    }

    if let Ok(bytes) = wrt.into_inner() {
        Ok(bytes)
    } else {
        Err(AppError::from("Could not parse csv"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ParentPaxCSVItem {
    /// f3 name of pax
    pub pax_name: String,
    /// optional slack id of pax
    pub slack_id: Option<String>,
    /// json string
    pub parent: String,
}

impl From<ParentPaxRelationDbItem> for ParentPaxCSVItem {
    fn from(value: ParentPaxRelationDbItem) -> Self {
        ParentPaxCSVItem {
            pax_name: value.pax_name.to_string(),
            slack_id: value.slack_id.clone(),
            parent: value.parent.to_string(),
        }
    }
}
