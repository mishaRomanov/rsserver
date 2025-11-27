mod app_state;
mod cfg;
mod handler;
mod jwt;
mod models;
mod postgres;
mod tools;

use app_state::AppState;
use axum::{middleware, routing, Extension};
use handler::Handlers;
use postgres::PostgresAccessor;
use tokio::net;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // Config parsing.
    let config = cfg::Config::new();

    // Initialize logging to stdout.
    tracing_subscriber::fmt::init();

    // Create postgres accessor instance.
    let pg_accessor = {
        match PostgresAccessor::new(config.db_addr).await {
            Ok(pg) => pg,
            Err(e) => panic!("failed to establish connection to a database: {e}"),
        }
    };

    // App state creation.
    let app_state = AppState::new(pg_accessor, jwt::TokenService::new(&config.jwt_secret)).await;
    match net::TcpListener::bind(&config.socket_addr).await {
        Ok(tcp_listener) => {
            tracing::info!("Listening on {}", &config.socket_addr);

            // Only accesible with JWT token.
            let authorized_router = axum::Router::new()
                .route("/log", routing::post(Handlers::receive_log))
                .route("/logs", routing::get(Handlers::list_logs))
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    tools::ServiceMiddleware::validate_token,
                ));

            let public_router = axum::Router::new()
                .route("/", routing::get(Handlers::root))
                .route("/auth", routing::post(Handlers::auth));

            let app = axum::Router::new()
                .merge(authorized_router)
                .merge(public_router)
                .layer(
                    ServiceBuilder::new()
                        .layer(middleware::from_fn(tools::ServiceMiddleware::log_request)),
                )
                .layer(Extension(app_state));

            axum::serve(tcp_listener, app).await.unwrap();
        }
        Err(e) => panic!("failed to bind tcp socket: {e}"),
    }
}
