use crate::models;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    username: String,
    email: String,
    expires: usize,
}

// TokenService does all all basic operations with JWT tokens.
// It also stores secret and header_algorithm in self.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenService {
    header_algorithm: Algorithm,
    encoding_secret: String,
}

impl TokenService {
    pub fn new(secret: String) -> Arc<Self> {
        Arc::new(Self {
            header_algorithm: Algorithm::HS256,
            encoding_secret: secret,
        })
    }

    pub fn from_user(&self, user: models::User) -> Result<String, String> {
        let header = jsonwebtoken::Header::new(self.header_algorithm);
        let claims = TokenClaims {
            username: user.name,
            email: user.email,
            expires: 10 as usize,
        };

        match jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.encoding_secret.as_bytes()),
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(format!("failed to create JWT token for user: {e}")),
        }
    }

    // Checks whether given token does exist and is valid.
    pub fn is_valid(&self, token: String) -> Result<bool, String> {
        match jsonwebtoken::decode::<TokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.encoding_secret.as_bytes()),
            &Validation::new(self.header_algorithm),
        ) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("failed to decode token: {e}")),
        }
    }
}
