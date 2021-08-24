use sqlx::SqlitePool;
use sqlx::Pool;
use crate::user::service::UserService;
use serde::{Serialize, Deserialize};
use actix_web::{web};

pub struct AppState {
    pub database_pool: SqlitePool,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(database_pool: SqlitePool) -> AppState {
        AppState {
            database_pool: database_pool.clone(),
            user_service: UserService::new(database_pool.clone())
        }
    }

    pub fn as_web_data(database_pool: SqlitePool) -> web::Data<AppState> {
        web::Data::new(AppState::new(database_pool))
    }
}

