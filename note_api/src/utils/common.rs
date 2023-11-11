use axum::{http::StatusCode, Json};


pub fn create_err_response(e:impl std::error::Error)->(StatusCode, Json<serde_json::Value>){
    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Database error: {}", e),
    });
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}