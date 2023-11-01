
use axum::{Router, routing::{post, get}, extract::State, Json, response::IntoResponse, http::StatusCode, Extension, middleware};
use serde_json::Value;

use crate::{AppState, utils::{AuthMiddleWare, auth}};

pub fn account_routes(app_state:AppState)->Router<AppState>{
    Router::new().route("/account/info",get(user_info))
        .route_layer(middleware::from_fn_with_state(app_state, auth))
}

async fn user_info(
    State(data): State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    println!("{:#?}",token_info);
    Ok((StatusCode::OK,Json(token_info)).into_response())
}