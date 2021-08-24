use sqlx::sqlite::SqlitePool;
use crate::db;
use crate::db::{DatabaseType};

pub async fn setup_db() -> SqlitePool {
    let pool = db::get_database_pool(DatabaseType::MEMORY).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    pool
}