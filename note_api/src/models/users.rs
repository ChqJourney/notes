use chrono::{DateTime, Local, Utc};
use garde::Validate;
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct UserClaim{
    pub id:Uuid,
    pub user_id:Uuid,
    pub claim_type:String,
    pub claim_value:String
}

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
pub struct RegisterModel {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 3, max = 50))]
    pub user_name: Option<String>,
    #[garde(length(min = 6, max = 50))]
    pub password: String,
}
#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
pub struct LoginModel {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 6, max = 50))]
    pub password: String,
}
#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
pub struct RefreshModel {
    #[garde(length(min = 6))]
    pub refresh_token: String,
}