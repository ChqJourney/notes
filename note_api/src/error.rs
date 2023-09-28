use axum::{response::IntoResponse, http::StatusCode};

#[derive(Debug)]
pub struct ApiError {
    details: String,
}
impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self {
            details: value.to_string(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // ...
        (StatusCode::INTERNAL_SERVER_ERROR, self.details).into_response()
    }
}
impl core::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ApiError {}