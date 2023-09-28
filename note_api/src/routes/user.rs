use axum::{Router, routing::get, response::IntoResponse};



pub fn user_routes()->Router{
    Router::new().route("/api/user",get(create_user))
}
async fn create_user()->impl IntoResponse{
    "user created".into_response()
}