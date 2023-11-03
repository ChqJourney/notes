use axum::{
    extract::{State, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::AppState;

use super::decode_token;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthMiddleWare {
    pub user_id: String,
    pub email: String,
    pub claims: Value,
}

pub async fn auth_common<B>(
    // run the `TypedHeader` extractor
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(data): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let validation_result =
        decode_token(auth.token().to_string(), data.inner.env.secret_key)
            .map_err(|e| {
            let error_response = json!( {
                "status": "fail",
                "message": "You are not logged in, please provide token".to_string(),
            });
            (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
        });
    let token_validate_result = validation_result.map_err(|e| {
        let error_response = json!( {
            "status": "fail",
            "message": "You are not logged in, please provide token".to_string(),
        });
        (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
    });
    if token_validate_result.is_ok(){

        let token_data = token_validate_result.unwrap().claims;
        request.extensions_mut().insert(AuthMiddleWare {
            user_id: token_data["sub"].as_str().unwrap().to_owned(),
            email: token_data["email"].as_str().unwrap().to_owned(),
            claims: token_data,
        });
        
        Ok(next.run(request).await)
    }else{
        Err((StatusCode::UNAUTHORIZED,Json(json!({}))))
    }
}

pub async fn auth_admin<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(data): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
)-> Result<impl IntoResponse, (StatusCode, Json<Value>)>{
    let validation_result =
        decode_token(auth.token().to_string(), data.inner.env.secret_key)
            .map_err(|e| {
            let error_response = json!( {
                "status": "fail",
                "message": "You are not logged in, please provide token".to_string(),
            });
            (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
        });
    let token_validate_result = validation_result.map_err(|e| {
        let error_response = json!( {
            "status": "fail",
            "message": "You are not logged in, please provide token".to_string(),
        });
        (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
    });
    if token_validate_result.is_ok(){

        let token_data = token_validate_result.unwrap().claims;
        let r=token_data.get("admin");
        println!("{:#?}",r);
        if token_data.get("admin").is_none(){
            return Err((StatusCode::UNAUTHORIZED,Json(json!({}))))
        }else{

            request.extensions_mut().insert(AuthMiddleWare {
                user_id: token_data["sub"].as_str().unwrap().to_owned(),
                email: token_data["email"].as_str().unwrap().to_owned(),
                claims: token_data,
            });
        }
        
        Ok(next.run(request).await)
    }else{
        Err((StatusCode::UNAUTHORIZED,Json(json!({}))))
    }
}
