use crate::{app_state::AppState, models};
use axum::{
    body::Body,
    extract::{Extension, Request},
    middleware::Next,
    response::Response,
};
use http::StatusCode;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceMiddleware();

impl ServiceMiddleware {
    pub async fn log_request(request: Request, next: Next) -> Response {
        tracing::info!(
            "{} request for {}",
            request.method().as_str(),
            request.uri().path(),
        );
        next.run(request).await
    }

    pub async fn validate_token(
        Extension(state): Extension<Arc<AppState>>,
        request: Request,
        next: Next,
    ) -> Response {
        if let Some(token) = request.headers().get("Authorization") {
            if let Ok(result) = state.jwt.is_valid(token.to_str().unwrap().to_string()) {
                match result {
                    true => next.run(request).await,
                    false => Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from(models::ErrorResponse::from_string(
                            &"invalid JWT token".to_string(),
                        )))
                        .unwrap(),
                }
            } else {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(models::ErrorResponse::from_string(
                        &"failed to decode JWT token".to_string(),
                    )))
                    .unwrap()
            }
        } else {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(models::ErrorResponse::from_string(
                    &"Authorization token not provided.".to_string(),
                )))
                .unwrap()
        }
    }
}
