use crate::postgres;
use std::sync::Arc;
// AppState contains information that is
// shared between all handlers. For example, dataabase access.
// It is, in a way, some kind of dependency injection.
#[derive(Clone)]
pub struct AppState {
    pub pg: postgres::PostgresAccessor,
}
impl AppState {
    pub async fn new(pg_accessor: postgres::PostgresAccessor) -> Arc<Self> {
        Arc::new(Self { pg: pg_accessor })
    }
}
