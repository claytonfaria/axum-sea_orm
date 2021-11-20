mod auth;
pub mod config;
mod entity;
mod error;
mod users;

#[macro_use]
extern crate lazy_static;

use std::time::Duration;

use auth::handlers::auth_routes;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, AddExtensionLayer,
    BoxError, Router,
};
use sea_orm::DatabaseConnection;

use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;
use users::handlers::user_routes;

pub fn app(conn: DatabaseConnection) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(10))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(conn));

    Router::new()
        .nest("/users", user_routes())
        .nest("/auth", auth_routes())
        .layer(middleware_stack)
}
fn handle_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
pub async fn handler_404() -> impl IntoResponse {
    tracing::info!("404");
    (StatusCode::NOT_FOUND, "Not available")
}
