use std::{time::Duration, sync::Arc};

use axum::{Router, error_handling::HandleErrorLayer, http::{Request, HeaderName, StatusCode, HeaderValue, Method, Uri}, body::{Body, BoxBody, boxed}, response::Response, BoxError, middleware, routing::get};
use axum_trace_id::SetTraceIdLayer;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer, classify::ServerErrorsFailureClass, propagate_header::PropagateHeaderLayer, cors::CorsLayer};
use tracing::Span;

use crate::{routes, utils::auth_common, AppState};

pub fn create_app(app_state:AppState)->Router{
    Router::new()
    .merge(routes::note_routes(app_state.clone()))
    .merge(routes::account_routes(app_state.clone()))
    .merge(routes::file_routes(app_state.clone()))
    .merge(routes::user_routes())
    .with_state(app_state)
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .layer(
            ServiceBuilder::new()
                // handle error outside routes
                .layer(HandleErrorLayer::new(handle_global_error))
                // trace all request and response
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(|request: &Request<Body>, _span: &Span| {
                            // get ip and request_id for log
                            let request_id = request.headers().get("x-request-id").unwrap();
                            let ip = request.headers().get("X-Forwarded-For");

                            let ip = if let Some(real_ip) = ip {
                                let ip = real_ip.to_str().unwrap();
                                tracing::info!(
                                    "started '{}' request,id:{:#?}, at uri: '{}', from ip: '{}'",
                                    request.method(),
                                    request_id,
                                    request.uri().path(),
                                    ip
                                )
                            } else {
                                tracing::info!(
                                    "started '{}' request,id:{:#?}, at uri: '{}', from ip: '{}'",
                                    request.method(),
                                    request_id,
                                    request.uri().path(),
                                    "anonymous"
                                )
                            };
                        })
                        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                            // println!("{:#?}",response);
                            let request_id = response.headers().get("x-request-id").unwrap();
                            
                            // println!("id:{:#?}",request_id);
                            tracing::info!(
                                "response to {:?} status code {} generated in {:?}",
                                request_id,
                                response.status(),
                                latency
                            )
                        })
                        .on_failure(
                            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                                tracing::error!("error occurred:{:#?}", error)
                            },
                        ),
                )
                .layer(CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://www.photonee.com".parse::<HeaderValue>().unwrap(),
                    "http://www.onebitai.com".parse::<HeaderValue>().unwrap(),
                    ])
                .allow_methods([Method::GET,Method::POST,Method::DELETE,Method::PATCH,Method::PUT,Method::OPTIONS])
                .allow_credentials(true))
                // timeout layer,more than 10sec error
                .timeout(Duration::from_secs(10))
                // copy request id to response
                .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
                
        ).layer(SetTraceIdLayer::<String>::new().with_header_name("x-request-id"))
}

// outside routes error handler
async fn handle_global_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}


async fn static_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {

    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new(".").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}