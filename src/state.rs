use crate::db;
use sqlx::SqlitePool;

pub struct AppState {
    pub database_pool: SqlitePool
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            database_pool: db::get_database_pool("sqlite::memory:").await.unwrap()
        }
    }
}

