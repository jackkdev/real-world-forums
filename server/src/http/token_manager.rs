use anyhow::Result;
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::{token::Signed, AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha512;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

/// Provides a way to quickly generate and verify tokens.
#[derive(Clone)]
pub struct TokenManager {
    secret: Hmac<Sha512>,
}

impl TokenManager {
    pub fn new(secret: &String) -> Result<Self> {
        Ok(Self {
            secret: Hmac::new_from_slice(secret.as_bytes())?,
        })
    }

    pub fn generate(&self, sub: i32) -> Result<Token<Header, Claims, Signed>> {
        let header = Header {
            algorithm: AlgorithmType::Hs512,
            ..Default::default()
        };

        let claims = Claims {
            sub,
            exp: Utc::now().timestamp_millis(),
        };

        Ok(Token::new(header, claims).sign_with_key(&self.secret)?)
    }

    pub fn verify(&self, token: &String) -> Option<i32> {
        let contents: Token<Header, Claims, _> = match token.verify_with_key(&self.secret) {
            Ok(contents) => contents,
            Err(_) => return None,
        };

        if contents.claims().exp <= Utc::now().timestamp_millis() {
            None
        } else {
            Some(contents.claims().sub)
        }
    }
}
