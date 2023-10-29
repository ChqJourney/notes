use chrono::{NaiveDate, DateTime, Local, TimeZone};
use serde_json::{json, Value, Map};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};
#[derive(Clone)]
pub struct UserDbService {
    pub conn: Pool<Sqlite>,
}
impl UserDbService {
    pub async fn new(pool:Pool<Sqlite>)->Self{
        UserDbService {
            conn: pool,
        }
    }
    pub async fn ensure_db_created(){
        
    }
}