use crate::shared::common_errors::AppError;
use actix_web::HttpRequest;

pub fn valid_internal_request(req: &HttpRequest, boise_key: &str) -> Result<(), AppError> {
    if let Some(headers) = req.headers().get("X-F3Boise-Key") {
        let key = headers.to_str().unwrap_or_default();
        if key == boise_key {
            Ok(())
        } else {
            Err(AppError::from("Invalid Internal key"))
        }
    } else {
        Err(AppError::from("No F3 Boise key provided"))
    }
}
