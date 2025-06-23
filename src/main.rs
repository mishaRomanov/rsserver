mod app_state;
mod cfg;
mod handler;
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
            println!("Listening on {}\n\n", &config.socket_addr);
            println!("available endpoints:\nGET /\nPOST /log");

            axum::serve(
                tcp_listener,
                axum::Router::new()
                    .route("/", routing::get(Handlers::root))
                    .route("/log", routing::post(Handlers::receive_log))
                    .layer(Extension(app_state)),
            )
            .await
            .unwrap();
        }
        Err(e) => panic!("failed to bind tcp socket: {e}"),
    }
}
