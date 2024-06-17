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
