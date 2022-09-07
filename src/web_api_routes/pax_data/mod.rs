use crate::app_state::MutableAppState;
use crate::db::init::get_db_users;
use crate::users::f3_user::F3User;
use crate::web_api_state::MutableWebState;
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
    post_count: usize,
    q_count: usize,
    start_date: NaiveDate,
}

impl PaxInfoResponse {
    pub fn new() -> Self {
        PaxInfoResponse {
            ..Default::default()
        }
    }
}

impl Default for PaxInfoResponse {
    fn default() -> Self {
        PaxInfoResponse {
            post_count: 0,
            q_count: 0,
            start_date: NaiveDate::MAX,
        }
    }
}

pub async fn get_pax_info(
    _web_state: web::Data<MutableWebState>,
    _app_state: web::Data<MutableAppState>,
    _req: web::Query<PaxInfoQuery>,
) -> impl Responder {
    // TODO
    HttpResponse::Ok().finish()
    // match data.db.get_all_back_blast_data() {
    //     Ok(list) => {
    //         let user_name = {
    //             let app = data.app.lock().expect("Could not lock");
    //             app.users
    //                 .get(req.id.as_str())
    //                 .map(|user| user.name.to_string())
    //         };
    //         if user_name.is_none() {
    //             return HttpResponse::NotFound().body("User not found");
    //         }
    //
    //         let user_name = user_name.unwrap();
    //         let response = list
    //             .iter()
    //             .filter(|bb| bb.get_pax().contains(&user_name))
    //             .fold(PaxInfoResponse::new(), |mut acc, item| {
    //                 acc.post_count += 1;
    //                 if item.qs.contains(&user_name) {
    //                     acc.q_count += 1;
    //                 }
    //
    //                 if item.date < acc.start_date {
    //                     acc.start_date = item.date;
    //                 }
    //                 acc
    //             });
    //
    //         HttpResponse::Ok().json(response)
    //     }
    //     Err(err) => HttpResponse::NotFound().body(format!("Err: {:?}", err)),
    // }
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
