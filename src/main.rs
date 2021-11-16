mod config;
mod entity;

use config::{db::establish_connection_db, shutdown::shutdown_signal};

use axum_sea_orm::app;

use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_sea_orm=debug,tower_http=trace")
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let conn = establish_connection_db().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let server = axum::Server::bind(&addr)
        .serve(app(conn).into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
