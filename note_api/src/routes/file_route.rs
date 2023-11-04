use std::path::Path;

use axum::{Router, routing::{post, get}, extract::{State, Multipart}, Json, response::IntoResponse, http::StatusCode, Extension, middleware};

use serde_json::{Value, json};
use tokio::fs;

use crate::{AppState, utils::{AuthMiddleWare, auth_common, ClaimKV, auth_admin, create_err_response}, models::User};

pub fn file_routes(app_state:AppState)->Router<AppState>{
    Router::new().route("/file/upload",post(upload_file))
                // .route("/file/download", post(add_claims))
        .route_layer(middleware::from_fn_with_state(app_state, auth_common))
}
const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB
async fn upload_file(
    State(data): State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>,
    mut multipart: Multipart
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{:#?}",multipart);
    if let Some(data)=multipart.next_field().await.unwrap(){

        let name = data.name().unwrap().to_string();
        let data = data.bytes().await.unwrap();
        let path=create_absolute_path(name, token_info.user_id);
        println!("{}",path);
        fs::write(path, data).await
        .map_err(|e|{
            create_err_response(e)
        })?;
    }
    
    Ok((StatusCode::OK,Json({})).into_response())
}

fn create_absolute_path(file_name:String,user_id:String)->String{
    let path=format!("uploads/{}",user_id.clone());
    if !std::path::Path::new(&path.clone()).exists(){
        std::fs::create_dir(&path.clone()).unwrap();
    }
    format!("{path}/{file_name}")
}