use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};
use validator::{Validate, ValidationError};

#[allow(non_snake_case)]
#[derive(Debug,Validate, Deserialize, FromRow, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub photo: Option<String>,
    pub verified: bool,
    pub access_fail_count:u32,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub version:String,
    pub is_deleted:bool
}

#[derive(Deserialize,Serialize,Clone,Debug,Validate)]
pub struct RegisterModel{
    #[validate(email)]
    pub email:String,
    #[validate(length(min=4))]
    pub user_name:Option<String>,
    #[validate(length(min=6))]
    pub password:String
}
#[derive(Deserialize,Serialize,Clone,Debug,Validate)]
pub struct LoginModel{
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String
}