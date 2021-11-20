use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use super::error::AuthError;

lazy_static! {
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            email: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: String) -> Result<String, AuthError> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|e| AuthError::JwtError(e))?)
}

pub fn verify(token: &str) -> Result<Claims, AuthError> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AuthError::JwtError(e))?)
}
