use std::{net::SocketAddr, fs::{File, self}, sync::Arc};
use axum::extract::FromRef;
use dotenv;
use sqlx::{Sqlite, Pool, sqlite::SqlitePoolOptions};
use tracing_appender::rolling;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, fmt::writer::MakeWriterExt,
};

use crate::{app::create_app, config::Config};
mod app;
mod error;
mod routes;
mod models;
mod repos;
mod utils;
mod config;

#[derive(Debug, Clone,FromRef)]
pub struct AppState{
    inner:InnerState,
    without_validation_arguments: ()
}

#[derive(Debug, Clone)]
pub struct InnerState {
    db: Pool<Sqlite>,
    env: Config,
    
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config=Config::init();
    let pool = match SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    // let dd=rolling::hourly("");
    // log setting in production
    #[cfg(not(debug_assertions))]
    let debug_file = rolling::hourly("./logs", "debug");
    #[cfg(not(debug_assertions))]
    let warn_file = rolling::daily("./logs", "warnings").with_max_level(tracing::Level::WARN);
    #[cfg(not(debug_assertions))]
    let all_files = debug_file.and(warn_file);

    #[cfg(not(debug_assertions))]
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "note_api=info,tower_http=error".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(all_files)
                .with_ansi(false),
        )
        .init();
    // log setting in development
    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "note_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // app router with layers created
    let app = create_app(AppState { inner: InnerState { db: pool, env: config }, without_validation_arguments: () });

    // app serve at:
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("note api started, listening on {}", addr);
    
    // serving
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
