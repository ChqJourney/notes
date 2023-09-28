use std::{net::SocketAddr, time::Duration};

use axum::{
    body::{Body, HttpBody},
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method, Request, Response, StatusCode, Uri, HeaderName},
    response::{IntoResponse, Result},
    routing::get,
    BoxError, Json, Router, ServiceExt,
};
use axum_trace_id::SetTraceIdLayer;
use serde_json::Value;
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::ServeDir,
    trace::{self, DefaultMakeSpan, TraceLayer}, propagate_header::PropagateHeaderLayer,
};
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{info, Level, Span};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};
mod routes;
#[tokio::main]
async fn main() {
    #[cfg(not(debug_assertions))]
    let debug_file = rolling::hourly("./logs", "debug");
    // Log warnings and errors to a separate file. Since we expect these events
    // to occur less frequently, roll that file on a daily basis instead.
    #[cfg(not(debug_assertions))]
    let warn_file = rolling::daily("./logs", "warnings").with_max_level(tracing::Level::WARN);
    #[cfg(not(debug_assertions))]
    let all_files = debug_file.and(warn_file);

    #[cfg(not(debug_assertions))]
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "note_api=debug,tower_http=debug".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(all_files)
                .with_ansi(false),
        )
        .init();
    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "note_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .merge(routes::routes())
        .nest_service("/", ServeDir::new("static"))
        .route("/api/users", get(users_show))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_global_error))
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(|request: &Request<Body>, _span: &Span| {
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
                .timeout(Duration::from_secs(10))
                .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
                
        ).layer(SetTraceIdLayer::<String>::new().with_header_name("x-request-id"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("note api started, listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn users_show() -> Result<StatusCode, ApiError> {
    make_error()?;
    Ok(StatusCode::OK)
}
fn make_error() -> Result<String, Box<dyn std::error::Error>> {
    let condition = false;
    if condition {
        Ok("ddd".into())
    } else {
        Err("eee".into())
    }
}
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

#[derive(Debug)]
struct ApiError {
    details: String,
}
impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self {
            details: value.to_string(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // ...
        (StatusCode::INTERNAL_SERVER_ERROR, self.details).into_response()
    }
}
impl core::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ApiError {}
