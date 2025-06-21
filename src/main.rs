mod app_state;
mod cfg;
mod handler;
mod models;

use app_state::AppState;
use axum::{routing, Extension};
use handler::Handlers;
use tokio::net;

#[tokio::main]
async fn main() {
    // Config parsing.
    let config = cfg::Config::new();

    // App state creation.
    let app_state = AppState::new(config.db_addr).await;

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
