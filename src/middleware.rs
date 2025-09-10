use axum::{extract::Request, middleware::Next, response::Response};
#[derive(Clone)]
pub struct ServiceMiddleware();
//
// pub struct JwtMiddleware();
//
// impl JwtMiddleware {
//     pub async fn validate_token(
//         State(state): State<app_state::AppState>,
//         request: Request,
//         next: Next,
//     ) -> Result<impl IntoResponse, Response> {
//         // TODO:
//     }
// }

impl ServiceMiddleware {
    pub async fn log_request(request: Request, next: Next) -> Response {
        tracing::info!(
            "{} request for {}",
            request.method().as_str(),
            request.uri().path(),
        );
        next.run(request).await
    }
}
