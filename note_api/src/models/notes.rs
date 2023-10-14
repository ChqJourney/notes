use chrono::{Local, NaiveDateTime, DateTime};
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
    
}

#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Tag{
    pub id:u32,
    pub created_on:DateTime<Local>,
    pub created_by:String,
    pub updated_on:DateTime<Local>,
    pub updated_by:String,
    pub tag_name:String,
    pub note_id:u32
}