use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};

#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Note{
    pub id:u32,
    pub created_on:DateTime<Local>,
    pub created_by:String,
    pub updated_on:DateTime<Local>,
    pub updated_by:String,
    pub title:String,
    pub board_id:u32,
    pub json_content:String
}