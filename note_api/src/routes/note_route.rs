use std::sync::Arc;

use axum::{
    body::{Body, HttpBody},
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method, Request, Response, StatusCode, Uri, HeaderName},
    response::{IntoResponse, Result},
    routing::get,
    BoxError, Json, Router, ServiceExt,
};

use crate::AppState;
pub fn note_routes()->Router<AppState>{
    Router::new().route("/api/note",get(create_note))
}

pub async fn create_note()->impl IntoResponse{
    "note created".into_response()
}