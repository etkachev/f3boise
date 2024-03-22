use crate::shared::common_errors::AppError;
use actix_web::HttpRequest;

pub const BOISE_KEY: &str = "f3-boise-208";

pub fn valid_internal_request(req: &HttpRequest) -> Result<(), AppError> {
    if let Some(headers) = req.headers().get("X-F3Boise-Key") {
        let key = headers.to_str().unwrap_or_default();
        if key == BOISE_KEY {
            Ok(())
        } else {
            Err(AppError::from("Invalid Internal key"))
        }
    } else {
        Err(AppError::from("No F3 Boise key provided"))
    }
}
