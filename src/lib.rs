pub mod config;
mod dto;
mod entity;
mod error;
pub mod handlers;

use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, routing::get, AddExtensionLayer, BoxError,
    Router,
};
use sea_orm::DatabaseConnection;

use tower_http::{auth::RequireAuthorizationLayer, trace::TraceLayer};

use tower::ServiceBuilder;

pub fn app(conn: DatabaseConnection) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(10))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(conn))
        .layer(RequireAuthorizationLayer::bearer("passwordlol"));

    Router::new()
        .route(
            "/users",
            get(handlers::get_all_users).post(handlers::create_user),
        )
        .route(
            "/users/:id",
            get(handlers::get_user)
                .delete(handlers::delete_user)
                .put(handlers::update_user),
        )
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
