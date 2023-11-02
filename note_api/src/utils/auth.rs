use std::collections::HashMap;

use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use base64::{engine::general_purpose, Engine};
use garde::Validate;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{pool::PoolOptions, Sqlite, FromRow, Pool};


pub fn hash_pwd(pwd:String)->Result<String,Box<dyn std::error::Error>>{
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .expect("hash password error");
    Ok(hashed_password.to_string())
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
#[derive(Serialize,Deserialize,Debug,Validate,FromRow)]
pub struct ClaimKV{
    #[garde(skip)]
    claim_type:String,
    #[garde(skip)]
    claim_value:String
}
pub async fn create_at_claims(user_id:String,email:String,pool:&Pool<Sqlite>,expires_in_minutes:i64)->Result<Value,Box<dyn std::error::Error>>{
    let now = chrono::Utc::now();
    let expire=(now+chrono::Duration::minutes(expires_in_minutes)).timestamp();
    let mut v=json!({"exp":expire,"iat":now.timestamp(),"iss":"note api","nbf":now.timestamp(),"sub":user_id,"email":email});
    let custom_claims=sqlx::query_as::<_,ClaimKV>("select * from userclaims where user_id=$1").bind(user_id).fetch_all(pool).await?;
    if custom_claims.len()==0{
        return Ok(v);
    }
    for claim in custom_claims {
        v.as_object_mut().unwrap().insert(claim.claim_type, serde_json::Value::String(claim.claim_value));
    }
    Ok(v)
}
pub async fn create_rt_claims(user_id:String,email:String,expires_in_hours:i64)->Result<Value,Box<dyn std::error::Error>>{
    let now = chrono::Utc::now();
    let expire=(now+chrono::Duration::hours(expires_in_hours)).timestamp();
    let v=json!({"exp":expire,"iat":now.timestamp(),"iss":"note api","nbf":now.timestamp(),"sub":user_id,"email":email});
    Ok(v)
}
pub fn generate_token(claims:Value,secret_str:String)->Result<String,Box<dyn std::error::Error>>{
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_str.as_bytes()))?;

    Ok(token)
    
}
pub fn decode_token(token:String,secret_str:String)->Result<TokenData<Value>,Box<dyn std::error::Error>>{
    let token = decode::<Value>(&token, &DecodingKey::from_secret(secret_str.as_ref()), &Validation::new(Algorithm::HS256))?;
    Ok(token)
}



