mod app_state;
mod cfg;
mod handler;
mod jwt;
// TODO: naming
mod middleware;
mod models;
mod postgres;

use app_state::AppState;
use axum::{routing, Extension};
use handler::Handlers;
use middleware::ServiceMiddleware as serviceMiddleware;
use postgres::PostgresAccessor;
use tokio::net;
use tower::ServiceBuilder;

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
    let tokens_service = jwt::TokenService::new(config.jwt_secret);
    let app_state = AppState::new(pg_accessor, tokens_service).await;
    match net::TcpListener::bind(&config.socket_addr).await {
        Ok(tcp_listener) => {
            tracing::info!("Listening on {}", &config.socket_addr);

            axum::serve(
                tcp_listener,
                axum::Router::new()
                    .route("/", routing::get(Handlers::root))
                    .route("/log", routing::post(Handlers::receive_log))
                    .route("/logs", routing::get(Handlers::list_logs))
                    .route("/auth", routing::post(Handlers::auth))
                    // Add middleware.
                    .layer(
                        ServiceBuilder::new()
                            .layer(axum::middleware::from_fn(serviceMiddleware::log_request)),
                    )
                    .layer(Extension(app_state)),
            )
            .await
            .unwrap();
        }
        Err(e) => panic!("failed to bind tcp socket: {e}"),
    }
}
