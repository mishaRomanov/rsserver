use crate::models;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    username: String,
    email: String,
    expires: u64,
}

// TokenService does all all basic operations with JWT tokens.
// It also stores secret and header_algorithm in self.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenService {
    header_algorithm: Algorithm,
    encoding_secret: String,
}

impl TokenService {
    pub fn new(secret: String) -> Self {
        Self {
            header_algorithm: Algorithm::HS256,
            encoding_secret: secret,
        }
    }

    pub fn from_user(&self, user: models::User) -> Result<String, String> {
        let header = jsonwebtoken::Header::new(self.header_algorithm);

        let hour = std::time::SystemTime::now()
            .checked_add(time::Duration::from_secs(3600))
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = TokenClaims {
            username: user.name,
            email: user.email,
            expires: hour,
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

    // TODO: add this shit to middleware.
    // Checks whether given token does exist and is valid.
    pub fn is_valid(&self, token: String) -> Result<bool, String> {
        // Create validation struct.
        let mut validation = Validation::new(self.header_algorithm);

        // skip exp validation, which is on by default.
        validation.required_spec_claims = HashSet::new();

        match jsonwebtoken::decode::<TokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.encoding_secret.as_bytes()),
            &validation,
        ) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("failed to decode token: {e}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::User;

    use super::*;

    #[test]
    fn test_token() {
        let service = TokenService::new("secret".to_string());
        let mock_user: User = User {
            name: "test_name".to_string(),
            email: "test_email".to_string(),
        };

        let token = service.from_user(mock_user).unwrap();
        assert_eq!(true, service.is_valid(token).unwrap());
    }
}
