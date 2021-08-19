use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn get_database_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let foo = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await;
    
    foo
}