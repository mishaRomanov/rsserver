use serde::{self, Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Log {
    // Should be formatted as rfc2822, otherwise parsing will fail.
    pub time: String,
    pub level: String,
    pub message: String,
}

impl Log {
    // parse to database-friendly format.
    pub fn parse_date_time(&self) -> Result<chrono::DateTime<chrono::Utc>, String> {
        match chrono::DateTime::parse_from_rfc2822(&self.time) {
            Ok(parsed_time) => Ok(parsed_time.to_utc()),
            Err(e) => Err(format!("failed to parse date from string: {e}")),
        }
    }
    pub fn response_from_vec(logs_vec: &Vec<Log>) -> Vec<u8> {
        serde_json::to_vec(&logs_vec).unwrap()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    // Returns ErrorResponse as bytes. Ready to be
    // passed into response::Body
    pub fn from_string(msg: &String) -> Vec<u8> {
        serde_json::to_vec(&Self { error: msg.clone() }).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}
