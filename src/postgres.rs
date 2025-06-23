use crate::models;
use sqlx::{postgres::PgPoolOptions, Executor};

// Wrapper over pg connection pool.
#[derive(Clone)]
pub struct PostgresAccessor {
    pg_pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresAccessor {
    pub async fn new(addr: String) -> Result<Self, String> {
        match PgPoolOptions::new()
            .max_connections(2) // let's keep it this way for now.
            .connect(addr.as_str())
            .await
        {
            Ok(pool) => Ok(Self { pg_pool: pool }),
            Err(e) => Err(format!("failed to connect to database: {e}")),
        }
    }

    pub async fn store_log(&self, payload: models::Log) -> Result<(), String> {
        let timestamp = payload.parse_date_time()?;

        match self
            .pg_pool
            .execute(
                sqlx::query("INSERT INTO logs (time, level, message) VALUES ($1,$2,$3)")
                    .bind(timestamp)
                    .bind(payload.level)
                    .bind(payload.message),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("failed to insert data in postgres: {e}")),
        }
    }
}
