use crate::app_state::ao_data::AO;
use crate::db::queries::q_line_up::get_single_q_line_up;
use crate::db::save_q_line_up::{save_list, NewQLineUpDbEntry};
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct ExternalQSignupRequest {
    pub ao_name: String,
    pub date: String, // YYYY-MM-DD
    pub q_slack_id: Option<String>,
    pub closed: bool,
}

#[derive(Serialize)]
pub struct ExternalQSignupResponse {
    pub success: bool,
    pub message: String,
}

/// Create Q signup from external source (F3 API)
/// This does NOT sync back to F3 API to avoid loops
pub async fn create_q_signup_from_external(
    db_pool: web::Data<PgPool>,
    payload: web::Json<ExternalQSignupRequest>,
) -> impl Responder {
    println!("Received external Q signup request: {:?}", payload);

    // 1. Parse AO from name
    let ao = AO::from(payload.ao_name.clone());

    if matches!(ao, AO::Unknown(_)) {
        return HttpResponse::BadRequest().json(ExternalQSignupResponse {
            success: false,
            message: format!("Unknown AO: {}", payload.ao_name),
        });
    }

    // 2. Parse date
    let date = match NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::BadRequest().json(ExternalQSignupResponse {
                success: false,
                message: format!("Invalid date format: {}", e),
            });
        }
    };

    // 3. Get channel_id for AO
    let channel_id = ao.channel_id().to_string();

    // 4. Check if spot already taken
    let already_exists = match get_single_q_line_up(&db_pool, &date, &channel_id).await {
        Ok(existing) => existing,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ExternalQSignupResponse {
                success: false,
                message: format!("Database error: {}", e),
            });
        }
    };

    if already_exists.is_some() {
        return HttpResponse::BadRequest().json(ExternalQSignupResponse {
            success: false,
            message: "Spot already taken".to_string(),
        });
    }

    // 5. Build DB entry
    let db_entry = if payload.closed {
        NewQLineUpDbEntry::new_closed(&ao, &date, &channel_id)
    } else {
        let q_name = match &payload.q_slack_id {
            Some(slack_id) => slack_id.clone(),
            None => {
                return HttpResponse::BadRequest().json(ExternalQSignupResponse {
                    success: false,
                    message: "q_slack_id required when not closing slot".to_string(),
                });
            }
        };
        NewQLineUpDbEntry::new(vec![q_name], &ao, &date, &channel_id)
    };

    // 6. Save to DB (no sync to F3 API)
    if let Err(e) = save_list(&db_pool, &[db_entry]).await {
        return HttpResponse::InternalServerError().json(ExternalQSignupResponse {
            success: false,
            message: format!("Failed to save Q signup: {}", e),
        });
    }

    println!("Successfully saved external Q signup for {} on {}", ao, date);

    HttpResponse::Ok().json(ExternalQSignupResponse {
        success: true,
        message: "Q signup saved successfully".to_string(),
    })
}
