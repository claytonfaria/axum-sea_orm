use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    JwtError(jsonwebtoken::errors::Error),
}

impl IntoResponse for AuthError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::JwtError(e) => {
                tracing::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "jwt error")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
