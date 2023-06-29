use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::MutableAppState;
use crate::db::queries::all_back_blasts::{get_all, get_dd_list_with_pax, get_list_with_pax};
use crate::db::queries::users::get_db_users;
use crate::users::f3_user::F3User;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

pub mod stats;

#[derive(Deserialize, Serialize)]
pub struct PaxInfoQuery {
    id: String,
}

#[derive(Serialize)]
pub struct PaxInfoResponse {
    pub name: String,
    pub post_count: usize,
    pub q_count: usize,
    pub start_date: NaiveDate,
    pub favorite_ao: FavoriteAoData,
}

/// represents a hashmap of ao and how many posts you did in that ao
#[derive(Default, Serialize)]
pub struct FavoriteAoData {
    data: HashMap<AO, u16>,
}

impl FavoriteAoData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn for_ao(&mut self, ao: &AO) {
        self.data
            .entry(ao.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    pub fn favorite(&self) -> String {
        if self.data.is_empty() {
            String::from("You need to first attend...")
        } else {
            self.data
                .iter()
                .max_by(|(_, num_a), (_, num_b)| num_a.cmp(num_b))
                .map(|(ao, _)| ao.to_string())
                .unwrap_or_else(|| "None".to_string())
        }
    }
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
            favorite_ao: FavoriteAoData::new(),
        }
    }
}

pub async fn get_pax_back_blasts(
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
        Ok(users) => {
            let data: Vec<BackBlastData> = users.into_iter().map(BackBlastData::from).collect();
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn get_pax_double_downs(
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

    match get_dd_list_with_pax(&db_pool, &user_name).await {
        Ok(users) => {
            let data: Vec<BackBlastData> = users.into_iter().map(BackBlastData::from).collect();
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
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
                    acc.favorite_ao.for_ao(&item.ao);
                    acc.post_count += 1;
                    if item.qs.contains(&user_name.to_lowercase()) {
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

#[derive(Serialize)]
pub struct BadUser {
    pub user: String,
    pub ao: String,
    pub date: NaiveDate,
}

#[derive(Serialize)]
pub struct BadUserDataResponse {
    pub slack_users: HashSet<String>,
    pub missing_pax: HashSet<String>,
    pub data: Vec<BadUser>,
}

impl BadUserDataResponse {
    pub fn new(slack_users: &HashSet<String>) -> Self {
        BadUserDataResponse {
            slack_users: slack_users.clone(),
            missing_pax: HashSet::new(),
            data: Vec::new(),
        }
    }
}

impl BadUser {
    pub fn new(user: String, bb: &BackBlastData) -> Self {
        BadUser {
            user,
            ao: bb.ao.to_string(),
            date: bb.date,
        }
    }
}

/// Collect some bad user data from backblasts.
pub async fn get_bad_data(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_db_users(&db_pool).await {
        Ok(users) => {
            let users = users
                .iter()
                .fold(HashSet::<String>::new(), |mut acc, (_, user)| {
                    acc.insert(user.name.to_lowercase());
                    acc
                });
            let mut response = BadUserDataResponse::new(&users);
            if let Ok(bb_list) = get_all(&db_pool).await {
                let list: Vec<BackBlastData> =
                    bb_list.into_iter().map(BackBlastData::from).collect();
                for item in list {
                    for bb_user in item.get_pax() {
                        if !users.contains(bb_user.as_str()) {
                            response.missing_pax.insert(bb_user.to_string());
                            response.data.push(BadUser::new(bb_user, &item));
                        }
                    }
                }
            }
            return HttpResponse::Ok().json(response);
        }
        Err(err) => {
            println!("Err: {:?}", err);
        }
    }
    HttpResponse::Ok().finish()
}
