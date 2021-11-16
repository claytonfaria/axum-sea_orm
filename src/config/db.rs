use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection_db() -> DatabaseConnection {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    return conn;
}
