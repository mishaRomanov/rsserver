mod app_state;
mod cfg;
mod handler;
mod jwt;
mod models;
mod postgres;

use app_state::AppState;
use axum::{routing, Extension};
use handler::Handlers;
use postgres::PostgresAccessor;
use tokio::net;

#[tokio::main]
async fn main() {
    // Config parsing.
    let config = cfg::Config::new();

    // Initialize logging to stdout.
    tracing_subscriber::fmt::init();

    let pg_accessor = {
        match PostgresAccessor::new(config.db_addr).await {
            Ok(pg) => pg,
            Err(e) => panic!("failed to establish connection to database: {e}"),
        }
    };

    // App state creation.
    let app_state = AppState::new(pg_accessor).await;

    match net::TcpListener::bind(&config.socket_addr).await {
        Ok(tcp_listener) => {
            tracing::info!("Listening on {}", &config.socket_addr);
            tracing::info!("available endpoints:\nGET /\nPOST /log\nGET /logs");

            axum::serve(
                tcp_listener,
                axum::Router::new()
                    .route("/", routing::get(Handlers::root))
                    .route("/log", routing::post(Handlers::receive_log))
                    .route("/logs", routing::get(Handlers::list_logs))
                    .layer(Extension(app_state)),
            )
            .await
            .unwrap();
        }
        Err(e) => panic!("failed to bind tcp socket: {e}"),
    }
}
