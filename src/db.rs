use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::config::{CONFIG};

pub async fn get_database_pool() -> SqlitePool {
    let config = CONFIG.clone();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap()
}