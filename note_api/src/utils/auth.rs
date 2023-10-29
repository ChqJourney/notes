use std::collections::HashMap;

use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{pool::PoolOptions, Sqlite, FromRow, Pool};

use crate::models::User;


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
#[derive(Serialize,Deserialize,FromRow)]
pub struct ClaimKV{
    claim_type:String,
    claim_value:String
}
pub async fn create_claims(user:User,pool:&Pool<Sqlite>)->Result<Value,Box<dyn std::error::Error>>{
    let now = chrono::Utc::now();
    let expire=(now+chrono::Duration::minutes(60)).timestamp();
    let mut v=json!({"exp":expire,"iat":now.timestamp(),"iss":"note api","nbf":now.timestamp(),"sub":user.id,"email":user.email});
    let custom_claims=sqlx::query_as::<_,ClaimKV>("select * from userclaims where user_id=$1").bind(user.id).fetch_all(pool).await?;
    if custom_claims.len()==0{
        return Ok(v);
    }
    for claim in custom_claims {
        v.as_object_mut().unwrap().insert(claim.claim_type, serde_json::Value::String(claim.claim_value));
    }
    Ok(v)
}
pub fn generate_token(claims:Value)->Result<String,Box<dyn std::error::Error>>{
    let token = encode(&Header::new(Algorithm::RS256), &claims, &EncodingKey::from_rsa_pem(include_bytes!("../private.pem"))?)?;
    Ok(token)
    
}
pub fn decode_token(token:String,key:&[u8])->Result<TokenData<Value>,Box<dyn std::error::Error>>{
    let token = decode::<Value>(&token, &DecodingKey::from_rsa_pem(include_bytes!("../public.pem"))?, &Validation::new(Algorithm::RS256))?;
    Ok(token)
}



