use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::json;

use crate::auth::jwt;

use super::{
    dto::{AuthBody, LoginInput},
    error::AuthError,
    service::AuthService,
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn register(Json(payload): Json<LoginInput>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(json!({"email": payload.email, "password": payload.password})),
    )
}
async fn login(Json(payload): Json<LoginInput>) -> Result<Json<AuthBody>, AuthError> {
    let user = AuthService::sign_in(payload).await?;

    let token = jwt::sign(user)?;

    Ok(Json(AuthBody::new(token)))
}
