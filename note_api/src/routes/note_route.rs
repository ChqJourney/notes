use std::sync::Arc;


use axum::{Router, routing::{post, get, put, delete}, extract::{State, Query}, Json, response::IntoResponse, http::StatusCode, Extension, middleware};
use axum_valid::Garde;
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, utils::{auth, AuthMiddleWare}, models::{Note, DeleteModel, QueryModel, NewNoteModel}};

pub fn note_routes(app_state:AppState)->Router<AppState>{
    Router::new().route("/biz/note",post(create_note))
                    .route("/biz/note", put(update_note))
                    .route("/biz/note",delete(delete_notes))
                    .route("/biz/note", get(query_notes))
    .route_layer(middleware::from_fn_with_state(app_state, auth))
}


async fn create_note(
    State(state):State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>,
    Garde(Json(note_model)):Garde<Json<NewNoteModel>>)
    ->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    
    let insert_result=sqlx::query("insert into notes(id,title,html_content,text_content,created_by)values($1,$2,$3,$4,$5)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(note_model.title)
        .bind(note_model.html_content)
        .bind(note_model.text_content)
        .bind(token_info.email)
        .execute(&state.inner.db)
        .await
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    if insert_result.rows_affected()==0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("save note failed"),
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
    }

    Ok(StatusCode::OK.into_response())
}
async fn update_note(
    State(state):State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>,
    Garde(Json(note)):Garde<Json<Note>>)
    ->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let update_result=sqlx::query("update notes set title=$1,html_content=$2,text_content=$3,updated_by=$4 where id=$5")
        .bind(note.title)
        .bind(note.html_content)
        .bind(note.text_content)
        .bind(token_info.email)
        .bind(note.id)
        .execute(&state.inner.db).await
        .map_err(|e|{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
        if update_result.rows_affected()==0 {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("save note failed"),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
        Ok(StatusCode::OK.into_response())
}
pub async fn query_notes(State(state):State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>,
    Garde(Query(query_model)):Garde<Query<QueryModel>>)
        ->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
            let start_sql=format!("select * from notes where ");
            let mut sql=Vec::new();
            if !query_model.content_contains.is_empty(){
                let sub_sql=format!("text_content like '%{}%'",query_model.content_contains);
                sql.push(sub_sql);
            }
            // if !query_model.title_contains.is_empty(){
            //     let sub_sql=format!("title like '%{}%'",query_model.title_contains);
            //     sql.push(sub_sql)
            // }
            // if !query_model.created_by.is_empty(){
            //     let sub_sql=format!("created_by='%{}%'",query_model.created_by);
            //     sql.push(sub_sql)
            // }
            let s=format!("{} {}",start_sql,sql.join(" and "));
            println!("{}",s);
            let query_result=sqlx::query_as::<_,Note>(&s)
                .fetch_all(&state.inner.db)
                .await.map_err(|e|{
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Database error: {}", e),
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                })?;

            Ok((StatusCode::OK,Json(json!(query_result))))
}
pub async fn delete_notes(State(state):State<AppState>,
    Extension(token_info):Extension<AuthMiddleWare>,
    Garde(Json(delete_model)):Garde<Json<DeleteModel>>)
        ->Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
            let mut sql=String::new();
            if delete_model.is_absolute_delete{
                sql.push_str("delete from notes where id=$1");
            }else{
                sql.push_str("update notes set is_deleted=1 where id=$1");
            }
        let delete_result=sqlx::query(&sql)
            .bind(delete_model.note_id)
            .execute(&state.inner.db).await
            .map_err(|e|{
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;
        if delete_result.rows_affected()==0{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("save note failed"),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
            Ok(StatusCode::OK.into_response())
}

