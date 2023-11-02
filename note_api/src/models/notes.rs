use chrono::{Local, NaiveDateTime, DateTime};
use garde::Validate;
use serde::{Serialize, Deserialize};
use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Decode, FromRow, Pool, Row, Sqlite, SqlitePool,
};


#[derive(Serialize, Deserialize,Validate,Debug,FromRow)]

pub struct Note{
    #[garde(skip)]
    pub id:String,
    #[garde(skip)]
    pub is_deleted:bool,
    #[garde(skip)]
    pub created_on:DateTime<Local>,
    #[garde(skip)]
    pub created_by:String,
    #[garde(skip)]
    pub updated_on:DateTime<Local>,
    #[garde(skip)]
    pub updated_by:String,
    #[garde(skip)]
    pub title:String,
    #[garde(skip)]
    pub html_content:String,
    #[garde(skip)]
    pub text_content:String,
    #[garde(skip)]
    pub version:String
}

#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Tag{
    pub id:String,
    pub is_deleted:bool,
    pub created_on:DateTime<Local>,
    pub created_by:String,
    pub updated_on:DateTime<Local>,
    pub updated_by:String,
    pub tag_name:String,
    pub note_id:u32
}
#[derive(Serialize, Deserialize,Validate, Clone, Debug)]
pub struct NewNoteModel{
    #[garde(length(min = 2))]
    pub title:String,
    #[garde(skip)]
    pub html_content:String,
    #[garde(skip)]
    pub text_content:String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryModel{
    pub created_by:Option<String>,
    pub title_contains:Option<String>,
    pub content_contains:Option<String>
}
#[derive(Serialize, Deserialize,Validate, Clone, Debug)]
pub struct DeleteModel{
    #[garde(skip)]
    pub note_id:u32,
    #[garde(skip)]
    pub is_absolute_delete:bool
}