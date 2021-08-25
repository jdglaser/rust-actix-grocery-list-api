use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use crate::config::get_config;

pub enum DatabaseType {
    MEMORY,
    FILE,
}

impl DatabaseType {
    pub fn from_str(value: &str) -> Result<DatabaseType, String> {
        match value {
            "memory" => Ok(DatabaseType::MEMORY),
            "file" => Ok(DatabaseType::FILE),
            _ => Err(format!("{} is not a valid DatabaseType", value))
        }
    }
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
        }
    };

    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&db_url)
        .await
        .unwrap()
}