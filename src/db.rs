use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use crate::config::get_config;

pub async fn get_database_pool() -> SqlitePool {
    let config = get_config();

    let db_url = match &*config.database_type {
        "memory" => "sqlite://:memory:",
        "file" => "sqlite://data/database.db",
        _ => panic!("Error: DATBASE_TYPE setting must be either 'memory' or 'file'")
    };

    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&db_url)
        .await
        .unwrap()
}