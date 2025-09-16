use crate::{jwt, postgres};
use std::sync::Arc;

// AppState contains information that is
// shared between all handlers. For example, database access.
// It is, in a way, some kind of dependency injection.
#[derive(Clone)]
pub struct AppState {
    pub pg: postgres::PostgresAccessor,
    pub jwt: jwt::TokenService,
}
impl AppState {
    pub async fn new(
        pg_accessor: postgres::PostgresAccessor,
        jwt_token_service: jwt::TokenService,
    ) -> Arc<Self> {
        Arc::new(Self {
            pg: pg_accessor,
            jwt: jwt_token_service,
        })
    }
}
