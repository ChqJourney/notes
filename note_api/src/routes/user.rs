use std::fs;

use axum::{Router, routing::get, response::IntoResponse};
use jwt_simple::prelude::*;


pub fn user_routes()->Router{
    Router::new().route("/api/user",get(create_user))
}
async fn create_user()->impl IntoResponse{
    let token=create_token();
    token.into_response()
}
fn create_token()->String{
    let dd=fs::read_to_string("private.pem").unwrap();
    let key = RS384KeyPair::from_pem(dd.as_str()).unwrap();
    let claims = Claims::create(Duration::from_hours(1)).with_subject("photonee.com").with_audience("onebitai.com").with_issuer("123");
    
    let token = key.sign(claims).unwrap();
    token
}
fn verify_token(token:String)->bool{
    let mut options = VerificationOptions::default();
// Accept tokens that will only be valid in the future
options.accept_future = true;
// Accept tokens even if they have expired up to 15 minutes after the deadline
// and/or they will be valid within 15 minutes.
options.time_tolerance = Some(Duration::from_mins(15));
// Reject tokens if they were issued more than 1 hour ago
options.max_validity = Some(Duration::from_hours(1));
// Reject tokens if they don't include an issuer from that list
options.allowed_issuers = Some(HashSet::from_strings(&["example app"]));
// See the documentation for the full list of available options
let dd=fs::read_to_string("public.pem").unwrap();
let key=RS384PublicKey::from_pem(dd.as_str()).unwrap();
let claims = key.verify_token::<Custom_Claims>(&token, Some(options)).unwrap();
if claims.custom.is_admin{
    true
}else{
    false
}
}

#[derive(Serialize, Deserialize)]
pub struct Custom_Claims{
    is_admin:bool
}