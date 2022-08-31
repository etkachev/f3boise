use crate::web_api_state::MutableWebState;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
            post_count: 0,
            q_count: 0,
            start_date: NaiveDate::MAX,
        }
    }
}

#[get("/")]
pub async fn get_pax_info(
    data: web::Data<MutableWebState>,
    req: web::Query<PaxInfoQuery>,
) -> impl Responder {
    match data.db.get_all_back_blast_data() {
        Ok(list) => {
            let user_name = {
                let app = data.app.lock().expect("Could not lock");
                app.users
                    .get(req.id.as_str())
                    .map(|user| user.name.to_string())
            };
            if user_name.is_none() {
                return HttpResponse::NotFound().body("User not found");
            }

            let user_name = user_name.unwrap();
            let response = list
                .iter()
                .filter(|bb| bb.get_pax().contains(&user_name))
                .fold(PaxInfoResponse::new(), |mut acc, item| {
                    acc.post_count += 1;
                    if item.qs.contains(&user_name) {
                        acc.q_count += 1;
                    }

                    if item.date < acc.start_date {
                        acc.start_date = item.date;
                    }
                    acc
                });

            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::NotFound().body(format!("Err: {:?}", err)),
    }
}
