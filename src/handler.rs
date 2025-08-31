use crate::models;
use crate::AppState;
use axum::{body::Body, extract::Extension, extract::Json, response::Response};
use http::StatusCode;
use tracing::instrument;

use std::sync::Arc;

// An empty struct as s container for all
// handlers to structure the project.
pub struct Handlers {}

impl Handlers {
    // Basic root handler.
    #[instrument]
    pub async fn root() -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-type", "application/json")
            .body(Body::from(r#"{"message": "hello, i am a root handler!"}"#))
            .unwrap()
    }

    // TODO: add /auth endpoint
    // and add token validation for each endpoint (maybe use axum middleware)

    // Receives log record and stores it into database.
    #[instrument(skip(state, payload))]
    pub async fn receive_log(
        Extension(state): Extension<Arc<AppState>>,
        Json(payload): Json<models::Log>,
    ) -> Response {
        match state.pg.store_log(payload).await {
            Ok(_) => {
                tracing::info!("STORE LOG SUCCESS");

                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(""))
                    .unwrap()
            }
            Err(e) => {
                tracing::error!("STORE LOG FAILED: {e}");

                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-type", "application/json")
                    .body(Body::from(models::ErrorResponse::from_string(&e)))
                    .unwrap()
            }
        }
    }

    #[instrument(skip(state))]
    pub async fn list_logs(Extension(state): Extension<Arc<AppState>>) -> Response {
        match state.pg.list_logs().await {
            Ok(logs) => {
                tracing::info!("LIST LOGS SUCCESS");

                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-type", "application/json")
                    .body(Body::from(models::Log::response_from_vec(&logs)))
                    .unwrap()
            }
            Err(e) => {
                tracing::error!("LIST LOGS FAILED: {e}");

                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-type", "application/json")
                    .body(Body::from(models::ErrorResponse::from_string(&e)))
                    .unwrap()
            }
        }
    }
}
