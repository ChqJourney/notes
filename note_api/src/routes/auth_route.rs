use std::{fs, sync::Arc};

use crate::AppState;
use crate::utils::{generate_token, create_claims};
use crate::{
    models::{TokenClaims, User},
    utils::hash_pwd,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_valid::{Garde, json};
use garde::{validate, Validate};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        
}

async fn register(
    State(data): State<AppState>,
    Garde(Json(register_model)): Garde<Json<RegisterModel>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let exists = sqlx::query_as::<_, User>("select * from users where email=$1 and is_deleted=0")
        .bind(register_model.email.clone())
        .fetch_one(&data.inner.db)
        .await;
    if exists.is_ok() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }
    let pwd_hash = hash_pwd(register_model.password).unwrap();
    let sqlite_result = sqlx::query(
        "insert into users(id,user_name,email,password_hash,version)values($1,$2,$3,$4,$5)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(register_model.user_name)
    .bind(register_model.email)
    .bind(pwd_hash)
    .bind(Uuid::new_v4().to_string())
    .execute(&data.inner.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
    });

    Ok(StatusCode::OK.into_response())
}

async fn login(
    State(data): State<AppState>,
    Json(login_model): Json<LoginModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let verify_result=verify_user(login_model, &data.inner.db).await
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "invalid email or password"
            });
            (StatusCode::BAD_REQUEST,Json(error_response))
        });
        let claims_result=create_claims(verify_result.unwrap(), &data.inner.db).await
            .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "invalid email or password"
            });
            (StatusCode::BAD_REQUEST,Json(error_response))
        });
        let token_result=generate_token(claims_result.unwrap())
            .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "invalid email or password"
            });
            (StatusCode::BAD_REQUEST,Json(error_response))
            });
        
    Ok((StatusCode::OK,Json(json!({"access_token":token_result.unwrap()}))))
}

async fn verify_user(
    login_model: LoginModel,
    pool: &Pool<Sqlite>,
) -> Result<User, Box<dyn std::error::Error>> {
    if let Ok(target_user) = sqlx::query_as::<_, User>("select * from users where email=$1 and is_deleted=0")
        .bind(login_model.email)
        .fetch_one(pool)
        .await
    {
        let is_pwd_match = verifiy_hashpwd(login_model.password, target_user.clone().password_hash);
        if is_pwd_match {
            Ok(target_user)
        } else {
            Err("email or password error".into())
        }
    } else {
        Err("email or password error".into())
    }
}
pub fn verifiy_hashpwd(password: String, hashpwd: String) -> bool {
    let is_valid = match PasswordHash::new(&hashpwd) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    is_valid
}
#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
pub struct RegisterModel {
    #[garde(email)]
    email: String,
    #[garde(ascii, length(min = 3, max = 50))]
    user_name: Option<String>,
    #[garde(length(min = 6, max = 50))]
    password: String,
}
#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
pub struct LoginModel {
    #[garde(email)]
    email: String,
    #[garde(length(min = 6, max = 50))]
    password: String,
}


