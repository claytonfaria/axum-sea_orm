use std::{env, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn establish_connection_db() -> DatabaseConnection {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");

    return conn;
}
