use std::time::Duration;

use axum::{Router, error_handling::HandleErrorLayer, http::{Request, HeaderName, StatusCode}, body::Body, response::Response, BoxError};
use axum_trace_id::SetTraceIdLayer;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer, classify::ServerErrorsFailureClass, propagate_header::PropagateHeaderLayer};
use tracing::Span;

use crate::routes;

pub fn create_app()->Router{
    Router::new()
        .merge(routes::user_routes())
        .merge(routes::note_routes())
        .nest_service("/api", ServeDir::new("static"))
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
                // timeout layer,more than 10sec error
                .timeout(Duration::from_secs(10))
                // copy request id to response
                .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
                
        )
        // add request-id to every request
        .layer(SetTraceIdLayer::<String>::new().with_header_name("x-request-id"))
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