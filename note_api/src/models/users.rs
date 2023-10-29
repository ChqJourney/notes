use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub photo: String,
    pub verified: bool,
    pub access_fail_count:u32,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub version:String,
    pub is_deleted:bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct UserClaim{
    pub id:Uuid,
    pub user_id:Uuid,
    pub claim_type:String,
    pub claim_value:String
}
