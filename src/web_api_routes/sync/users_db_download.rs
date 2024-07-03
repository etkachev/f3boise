use crate::db::queries::users::get_db_user_list;
use crate::db::save_user::DbUser;
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// download link to get csv of all users
pub async fn users_db_csv_download(db_pool: web::Data<PgPool>) -> impl Responder {
    match fetch_user_db_csv(&db_pool).await {
        Ok(results) => HttpResponse::Ok().body(results),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

async fn fetch_user_db_csv(db: &PgPool) -> Result<Vec<u8>, AppError> {
    let users = get_db_user_list(db).await?;
    let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
    // list map
    for item in users.into_iter() {
        if let Err(err) = wrt.serialize(UserCSVItem::from(item)) {
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
pub struct UserCSVItem {
    pub slack_id: String,
    pub name: String,
    pub email: String,
    pub img_url: Option<String>,
    pub parent: Option<String>,
    pub create_date: String,
}

impl From<DbUser> for UserCSVItem {
    fn from(value: DbUser) -> Self {
        UserCSVItem {
            slack_id: value.slack_id.to_string(),
            name: value.name.to_string(),
            email: value.email.to_string(),
            img_url: value.img_url.clone(),
            parent: value.parent.map(|p| p.to_string()),
            create_date: value.create_date.to_string(),
        }
    }
}
