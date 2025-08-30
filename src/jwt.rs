use axum::extract::rejection::JsonRejection;
//TODO: написать компонент по созданию и проверке jwt токенов + придумать как очищать сессии в базе
//и удалять старые токены
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};

const HEADER_ALGORITHM: Algorithm = Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    username: String,
    email: String,
    expires: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(username: String) -> Result<Self, String> {
        let header = jsonwebtoken::Header::new(HEADER_ALGORITHM);
        let claims = TokenClaims {
            username: username,
            email: "".to_string(),
            expires: 10 as usize,
        };

        match jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("hello".as_bytes()),
        ) {
            Ok(token) => Ok(Self { token: token }),
            Err(e) => Err(format!("failed to create JWT token for user: {e}")),
        }
    }
}
