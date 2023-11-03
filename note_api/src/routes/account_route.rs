
use axum::{Router, routing::{post, get}, extract::State, Json, response::IntoResponse, http::StatusCode, Extension, middleware};
use axum_valid::Garde;
use serde_json::{Value, json};
use sqlx::{Sqlite, Pool, Acquire};

use crate::{AppState, utils::{AuthMiddleWare, auth_common, ClaimKV, auth_admin}, models::User};

pub fn account_routes(app_state:AppState)->Router<AppState>{
    Router::new().route("/account/info",get(user_info))
                .route("/account/claims", post(add_claims))
        .route_layer(middleware::from_fn_with_state(app_state, auth_admin))
}

async fn user_info(
    State(data): State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user=sqlx::query_as::<_,User>("select * from users where id=$1 and is_deleted=0")
        .bind(token_info.user_id.clone())
        .fetch_one(&data.inner.db).await
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
        let custom_claims=sqlx::query_as::<_,ClaimKV>("select * from userclaims where user_id=$1")
            .bind(token_info.user_id).fetch_all(&data.inner.db)
            .await.map_err(|e|{
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
    let mut j_user=json!(user);
    for claim in custom_claims {
        j_user.as_object_mut().unwrap().insert(claim.claim_type, serde_json::Value::String(claim.claim_value));
    }
    Ok((StatusCode::OK,Json(j_user)).into_response())
}
async fn add_claims(State(data): State<AppState>,
Extension(token_info):Extension<AuthMiddleWare>,
Garde(Json(claims)):Garde<Json<Vec<ClaimKV>>>)->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    println!("{:#?}",claims);
    let mut tr=data.inner.db.begin().await.map_err(|e|{
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let mut affect_rows=0;
    for claim in claims.clone(){
        let insert_result=sqlx::query::<Sqlite>("insert into userclaims (user_id,claim_type,claim_value) values($1,$2,$3)")
        .bind(token_info.user_id.clone())
        .bind(claim.claim_type)
        .bind(claim.claim_value)
        .execute(&mut *tr).await
        .map_err(|e|{
            
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
        affect_rows=affect_rows+insert_result.rows_affected();
    }
    
    if affect_rows<claims.len().try_into().unwrap(){
        tr.rollback().await
            .map_err(|e|{
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
    }else{

        tr.commit().await.map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    }
    Ok(StatusCode::OK)
}