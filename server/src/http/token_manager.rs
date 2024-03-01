use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{token::Signed, AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha512;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,
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

    pub fn generate(&self, sub: u64) -> Result<Token<Header, Claims, Signed>> {
        let header = Header {
            algorithm: AlgorithmType::Hs512,
            ..Default::default()
        };

        let claims = Claims { sub };

        Ok(Token::new(header, claims).sign_with_key(&self.secret)?)
    }

    pub fn verify(&self, token: &String) -> Option<u64> {
        let contents: Token<Header, Claims, _> = match token.verify_with_key(&self.secret) {
            Ok(contents) => contents,
            Err(err) => return None,
        };

        Some(contents.claims().sub)
    }
}
