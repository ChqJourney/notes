use chrono::{NaiveDate, DateTime, Local, TimeZone};
use serde_json::{json, Value, Map};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};
#[derive(Clone)]
pub struct UserDbService {
    pub conn: SqlitePool,
}
impl UserDbService {
    pub async fn new(path:&str)->Self{
        UserDbService {
            conn: SqlitePool::connect(path).await.unwrap(),
        }
    }
}