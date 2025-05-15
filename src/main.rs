mod cfg;

use axum::{
    body::Body,
    response::{self, Response},
    routing,
};
use http::StatusCode;
use tokio::net;

#[tokio::main]
async fn main() {
    let config = cfg::Config::new();

    let tcp_listener = net::TcpListener::bind(&config.socket_addr)
        .await
        .expect("Failed to bind TCP socket");

    println!("Listening on {}", &config.socket_addr);

    axum::serve(
        tcp_listener,
        axum::Router::new().route("/", routing::get(root)),
    )
    .await
    .unwrap();
}

async fn root() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-type", "application/json")
        .body(Body::from(r#"{"message": "hello, i am a root handler!"}"#))
        .unwrap()
}
