// use chrono::{Local, NaiveDateTime, DateTime};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Debug)]
pub struct Note{
    pub id:u32,
    // pub created_on:DateTime<Local>,
    pub created_by:String,
    // pub updated_on:DateTime<Local>,
    pub updated_by:String,
    pub title:String,
    
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Tag{
    pub id:u32,
    // pub created_on:DateTime<Local>,
    pub created_by:String,
    // pub updated_on:DateTime<Local>,
    pub updated_by:String,
    pub tag_name:String,
    pub note_id:u32
}