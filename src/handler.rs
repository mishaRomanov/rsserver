use crate::AppState;

use axum::{body::Body, extract::Extension, response::Response};
use http::StatusCode;

use std::sync::Arc;

// An empty struct as s container for all
// handlers to structure the project.
pub struct Handlers {}

impl Handlers {
    // Basic root handler.
    pub async fn root() -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-type", "application/json")
            .body(Body::from(r#"{"message": "hello, i am a root handler!"}"#))
            .unwrap()
    }

    // Temporary handler to test Extension usage.
    pub async fn hello(Extension(state): Extension<Arc<AppState>>) -> Response {
        match state.pg.write_data().await {
            Ok(_) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-type", "application/json")
                .body(Body::from(r#"{"text":"hello!"}"#))
                .unwrap(),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-type", "application/json")
                .body(Body::from(r#"{"error":"#.to_string() + e.as_str() + r#"}"#))
                .unwrap(),
        }
    }
}
