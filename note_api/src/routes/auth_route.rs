use std::{fs, sync::Arc};

use crate::models::{LoginModel, RegisterModel, RefreshModel};
use crate::utils::{create_at_claims, create_rt_claims, generate_token, decode_token};
use crate::AppState;
use crate::{
    models::{User},
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
use axum_valid::{json, Garde};
use garde::{validate, Validate};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
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
    let verify_result = verify_user(login_model, &data.inner.db).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "invalid email or password"
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;
    let token_obj=create_tokens(data, verify_result.id,verify_result.email).await
    .map_err(|e|{
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "invalid email or password"
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;
        
    
    Ok((
        StatusCode::OK,
        Json(token_obj),
    ))
}


async fn refresh(State(data): State<AppState>,
Json(refresh_model): Json<RefreshModel>,)->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let token_data=decode_token(refresh_model.refresh_token, data.clone().inner.env.secret_key)
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "invalid email or password"
            });
            (StatusCode::BAD_REQUEST,Json(error_response))
        })?;
       let email=token_data.claims["email"].as_str().unwrap().to_owned();
       let user_id=token_data.claims["sub"].as_str().unwrap().to_owned();
        let token_obj=create_tokens(data, user_id,email).await
            .map_err(|e|{
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "you need to privide correct token"
                });
                (StatusCode::BAD_REQUEST,Json(error_response))
            })?;
        Ok((
            StatusCode::OK,
            Json(token_obj)
        ))
}

async fn create_tokens(state:AppState,user_id:String,email:String)->Result<Value,Box<dyn std::error::Error>>{
    let at_claims_result = create_at_claims(user_id.clone(),email.clone(), &state.inner.db,state.inner.env.jwt_expires_in)
        .await?;
        // println!("{:#?}",at_claims_result);
    let at_token_result = generate_token(at_claims_result, state.inner.env.secret_key.clone())?;
        println!("{:#?}",at_token_result);
    let rt_claims_result = create_rt_claims(user_id,email,state.inner.env.refresh_expires_in)
        .await?;
    let rt_token_result = generate_token(rt_claims_result, state.inner.env.secret_key)?;
        Ok(json!({
            "type":"Bearer",
            "access_token":at_token_result,
            "refresh_token":rt_token_result,
            "expires_in":state.inner.env.jwt_expires_in
        }))
}

 async fn verify_user(
    login_model: LoginModel,
    pool: &Pool<Sqlite>,
) -> Result<User, Box<dyn std::error::Error>> {
    if let Ok(target_user) =
        sqlx::query_as::<_, User>("select * from users where email=$1 and is_deleted=0")
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

 fn verifiy_hashpwd(password: String, hashpwd: String) -> bool {
    let is_valid = match PasswordHash::new(&hashpwd) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    is_valid
}



