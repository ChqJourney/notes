use argon2::{PasswordHash, Argon2, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}, PasswordHasher, Error};
use crate::models::{RegisterModel, LoginModel, User};
use serde_json::{json, value, Value};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, query};
use uuid::{uuid, Uuid};
use validator::Validate;
use core::result::Result::Ok;

pub async fn add_user(
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


pub fn validate_register_model(register_model: &RegisterModel) -> Result<bool,Box<dyn std::error::Error>> {
    match register_model.validate() {
        Ok(_) => return Ok(true),
        Err(e) => {
            return  Err(e.into());
        }
    }
}
pub fn hash_pwd(pwd:String)->Result<String,Box<dyn std::error::Error>>{
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .expect("hash password error");
    Ok(hashed_password.to_string())
}

pub async fn login_user(login_model:LoginModel,pool:&Pool<Sqlite>)->Result<Value,Box<dyn std::error::Error>>{
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
pub fn verifiy_hashpwd(password:String,hashpwd:String)->bool{
    let is_valid = match PasswordHash::new(&hashpwd) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    is_valid
}