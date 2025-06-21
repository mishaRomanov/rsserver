use std::str::FromStr;

use serde::{self, Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Deserialize)]
pub struct Log {
    pub time: String,
    pub level: String,
    pub message: String,
}

impl Log {
    pub fn parse_date_time(&self) -> Result<chrono::DateTime<chrono::Utc>, String> {
        match chrono::DateTime::from_str(self.time.as_str()) {
            Ok(parsed_time) => Ok(parsed_time),
            Err(e) => Err(format!("failed to parse date from string: {e}")),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    // Returns ErrorResponse as bytes. Ready to be
    // passed into axum::response::Body
    pub fn from_string(msg: &String) -> Vec<u8> {
        serde_json::to_vec(&Self { error: msg.clone() }).unwrap()
    }
}
