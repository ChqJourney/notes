use axum::{
    Router,
    extract::TypedHeader,
    http::StatusCode,
    headers::authorization::{Authorization, Bearer},
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};

pub async fn auth<B>(
    // run the `TypedHeader` extractor
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if token_is_valid(auth.token()) {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn token_is_valid(token:&str)->bool{
    true
}