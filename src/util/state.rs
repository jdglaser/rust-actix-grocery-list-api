use sqlx::SqlitePool;
use crate::user::service::UserService;

pub struct AppState {
    pub database_pool: SqlitePool,
    pub user_service: UserService,
}

impl AppState {
    pub async fn new(database_pool: SqlitePool) -> AppState {
        AppState {
            database_pool: database_pool.clone(),
            user_service: UserService::new(database_pool.clone())
        }
    }
}

