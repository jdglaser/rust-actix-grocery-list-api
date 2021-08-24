use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use crate::config::get_config;

pub enum DatabaseType {
    MEMORY,
    FILE,
}

pub async fn get_database_pool(database_type: DatabaseType) -> SqlitePool {
    let config = get_config();

    let db_url = match database_type {
        DatabaseType::MEMORY => {
            info!("Using in memory database");
            "sqlite://:memory:"
        },
        DatabaseType::FILE => {
            info!("Using database file");
            "sqlite://data/database.db"
        },
        _ => panic!("Error: DATBASE_TYPE setting must be either 'memory' or 'file'")
    };

    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&db_url)
        .await
        .unwrap()
}