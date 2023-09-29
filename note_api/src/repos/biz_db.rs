// use chrono::{NaiveDate, DateTime, Local, TimeZone};
use serde_json::{json, Value, Map};
// use sqlx::{
//     sqlite::{SqliteQueryResult, SqliteRow},
//     Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
// };

#[derive(Clone)]
pub struct BizDbService {
    pub conn: String,
}
