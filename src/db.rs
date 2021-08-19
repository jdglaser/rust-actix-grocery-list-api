use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use anyhow::Result;

pub async fn get_database_pool(database_url: &str) -> Result<SqlitePool> {
    Ok(SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?)
}