use crate::shared::common_errors::AppError;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    success: bool,
}

impl SuccessResponse {
    pub fn ok() -> Self {
        SuccessResponse { success: true }
    }

    pub fn err() -> Self {
        SuccessResponse { success: false }
    }
}

/// sends generic successful response
pub fn success() -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse::ok())
}

pub fn failure(err: AppError) -> HttpResponse {
    HttpResponse::BadRequest().body(err.to_string())
}
