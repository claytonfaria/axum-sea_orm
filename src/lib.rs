pub mod config;
mod dto;
mod entity;
mod error;
mod handlers;

use axum::{routing::get, AddExtensionLayer, Router};
use sea_orm::DatabaseConnection;

pub fn app(conn: DatabaseConnection) -> Router {
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
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(conn))
}
