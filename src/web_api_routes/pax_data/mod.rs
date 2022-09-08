use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::MutableAppState;
use crate::db::init::get_db_users;
use crate::db::queries::all_back_blasts::get_list_with_pax;
use crate::users::f3_user::F3User;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct PaxInfoQuery {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct PaxInfoResponse {
    name: String,
    post_count: usize,
    q_count: usize,
    start_date: NaiveDate,
}

impl PaxInfoResponse {
    pub fn new(name: &str) -> Self {
        PaxInfoResponse {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Default for PaxInfoResponse {
    fn default() -> Self {
        PaxInfoResponse {
            name: String::new(),
            post_count: 0,
            q_count: 0,
            start_date: NaiveDate::MAX,
        }
    }
}

pub async fn get_pax_info(
    db_pool: web::Data<PgPool>,
    app_state: web::Data<MutableAppState>,
    req: web::Query<PaxInfoQuery>,
) -> impl Responder {
    let user_name = {
        let app = app_state.app.lock().expect("Could not lock app");
        app.users
            .get(req.id.as_str())
            .map(|user| user.name.to_string())
    };

    if user_name.is_none() {
        return HttpResponse::NotFound().body("User not found");
    }

    let user_name = user_name.unwrap();

    match get_list_with_pax(&db_pool, &user_name).await {
        Ok(list) => {
            let response = list.into_iter().map(BackBlastData::from).fold(
                PaxInfoResponse::new(&user_name),
                |mut acc, item| {
                    acc.post_count += 1;
                    if item.qs.contains(&user_name) {
                        acc.q_count += 1;
                    }

                    if item.date < acc.start_date {
                        acc.start_date = item.date;
                    }
                    acc
                },
            );

            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let response = get_db_users(&db_pool).await;
    match response {
        Ok(users) => {
            let users: Vec<F3User> = users.into_values().collect();
            HttpResponse::Ok().json(users)
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
