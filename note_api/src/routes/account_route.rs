
use axum::{Router, routing::{post, get}, extract::State, Json, response::IntoResponse, http::StatusCode, Extension, middleware};
use axum_valid::Garde;
use serde_json::Value;

use crate::{AppState, utils::{AuthMiddleWare, auth, ClaimKV}};

pub fn account_routes(app_state:AppState)->Router<AppState>{
    Router::new().route("/account/info",get(user_info))
                .route("/account/claims", post(add_claims))
        .route_layer(middleware::from_fn_with_state(app_state, auth))
}

async fn user_info(
    State(data): State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    println!("{:#?}",token_info);
    Ok((StatusCode::OK,Json(token_info)).into_response())
}
async fn add_claims(State(data): State<AppState>,
Extension(token_info):Extension<AuthMiddleWare>,
Garde(Json(claims)):Garde<Json<Vec<ClaimKV>>>)->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    println!("{:#?}",claims);
    // let tr=data.inner.db.begin().await.map_err(|e|{
    //     let error_response = serde_json::json!({
    //         "status": "fail",
    //         "message": format!("Database error: {}", e),
    //     });
    //     (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    // })?;
    let insert_result=sqlx::query("insert into userclaims () values()")
        .execute(&data.inner.db).await
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    Ok(StatusCode::OK)
}