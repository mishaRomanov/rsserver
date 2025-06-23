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

    // Store log in a database.
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

    // List all logs.
    // TODO: fix time parsing stuff.
    // i should not convert it to text, but
    // serde does not support chrono timestamps for serialization.
    pub async fn list_logs(&self) -> Result<Vec<models::Log>, String> {
        match sqlx::query_as::<_, models::Log>("SELECT level, message, time::text FROM logs")
            .fetch_all(&self.pg_pool)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => Err(format!("failed to fetch all logs from database: {e}")),
        }
    }
}
