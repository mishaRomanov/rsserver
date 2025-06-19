use axum::{body::Body, extract::Extension, response::Response};
use http::StatusCode;

use std::sync::Arc;

// Contains storage that should be accesible
// by all handlers.
pub struct HandersState {
    pub vec_storage: Vec<String>,
}

impl HandersState {
    pub fn new() -> Arc<Self> {
        let mut fresh_vec: Vec<String> = vec![];
        fresh_vec.push("Hello!".to_string());

        Arc::new(HandersState {
            vec_storage: fresh_vec,
        })
    }
}

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
    pub async fn hello(Extension(state): Extension<Arc<HandersState>>) -> Response {
        let str_res = state
            .vec_storage
            .get(0)
            .expect("failed to get value from vector");

        let body: String = r#"{"message": "#.to_string() + str_res + r#"}"#;
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}
