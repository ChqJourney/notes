use argon2::{PasswordHash, Argon2, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}, PasswordHasher, Error};
use model::{RegisterModel, LoginModel};
use serde::{Serialize, Deserialize};
use serde_json::{json, value, Value};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, query};
use tracing_subscriber::registry;
use uuid::{uuid, Uuid};
use validator::Validate;
use core::result::Result::Ok;
use std::str::FromStr;
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, decode, DecodingKey, Validation};
use crate::model::User;
mod model;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {   
    // aud:String,   // Optional. Audience
    exp: i64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: i64,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
    nbdsd:Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Utc::now();
    let expire=(now+chrono::Duration::minutes(60)).timestamp();
    let mut claims=json!({"exp":expire,"iat":now.timestamp(),"iss":"dfd","nbf":now.timestamp(),"sub":"dfadf"});
    claims.as_object_mut().unwrap().insert("new".to_owned(),json!("adfadf"));
    
    println!("{:#?}", claims["iss"].as_str().unwrap());
    Ok(())
}

async fn add_user(
    register_model: RegisterModel,
    pool: &Pool<Sqlite>,
) -> Result<u64, Box<dyn std::error::Error>> {
    let exists=sqlx::query_as::<_,User>("select * from users where email=$1 and is_deleted=0").bind(register_model.email.clone()).fetch_one(pool).await;
    if exists.is_ok(){
        return  Err("user existed".into());
    }
    let c = sqlx::query(
        "insert into users(Id,user_name,email,password_hash,version)values($1,$2,$3,$4,$5)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(register_model.user_name)
    .bind(register_model.email)
    .bind(register_model.password)
    .bind(Uuid::new_v4().to_string())
    .execute(pool)
    .await?;

    Ok(c.rows_affected())
}


fn validate_register_model(register_model: &RegisterModel) -> bool {
    match register_model.validate() {
        Ok(_) => return true,
        Err(e) => {
            println!("{:#?}", e);
            return false;
        }
    }
}
fn hash_pwd(pwd:String)->Result<String,Box<dyn std::error::Error>>{
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .expect("hash password error");
    Ok(hashed_password.to_string())
}

async fn login_user(login_model:LoginModel,pool:&Pool<Sqlite>)->Result<Value,Box<dyn std::error::Error>>{
    if let Err(e)= login_model.validate(){
       return  Err(e.into());
    }
    let target_user=sqlx::query_as::<_,User>("sql").bind(login_model.email).fetch_one(pool).await;
    if let Ok(user)=target_user{
        let is_valid = match PasswordHash::new(&login_model.password) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(login_model.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true),
            Err(_) => false,
        };
    }else{
        return  Err("user name or email error".into());
    }
    Ok(json!({}))
}
fn verifiy_hashpwd(password:String,hashpwd:String)->bool{
    let is_valid = match PasswordHash::new(&hashpwd) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    is_valid
}