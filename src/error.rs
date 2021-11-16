use axum::{http::StatusCode, Json};
use sea_orm::DbErr;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error("wrong password")]
    WrongPassword,
    #[error("not found")]
    NotFound,
    #[error("failed to create user")]
    FailedCreateUser,
    // #[error("email is already taken")]
    // DuplicateUserEmail,
    // #[error("name is already taken")]
    // DuplicateUserName,
}

pub type ApiError = (StatusCode, Json<Value>);

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            // Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            // Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});

        tracing::error!("{}", err);
        (status, Json(payload))
    }
}
