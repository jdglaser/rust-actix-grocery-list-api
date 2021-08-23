use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use crate::config::get_config;

pub async fn get_database_pool() -> SqlitePool {
    let config = get_config();

    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .unwrap()
}