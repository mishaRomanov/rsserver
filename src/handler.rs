use crate::models;
use crate::AppState;
use axum::{body::Body, extract::Extension, extract::Json, response::Response};
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
    // Receives log record and stores it into database.
    pub async fn receive_log(
        Extension(state): Extension<Arc<AppState>>,
        Json(payload): Json<models::Log>,
    ) -> Response {
        match state.pg.store_log(payload).await {
            Ok(_) => Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(""))
                .unwrap(),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-type", "application/json")
                .body(Body::from(models::ErrorResponse::from_string(&e)))
                .unwrap(),
        }
    }
}
