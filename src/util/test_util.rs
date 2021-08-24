use sqlx::sqlite::SqlitePool;
use crate::db;
use crate::db::{DatabaseType};
use actix_web::{test, App, web, dev};
use crate::user::service::UserService;
use crate::user::{User, UserTemplate};
use crate::state::AppState;
use crate::config_app;
use serde::{de::DeserializeOwned};

pub async fn setup_test_db() -> SqlitePool {
    let pool = db::get_database_pool(DatabaseType::MEMORY).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    pool
}

pub async fn login_test_user(req: test::TestRequest, db: &SqlitePool) -> test::TestRequest {
    let test_user = UserTemplate::new("JohnDoe".to_string(), "1234".to_string());
    let user_service = UserService::new(db.clone());
    let check_for_user = User::find_user_by_username(db, &test_user.username).await;

    let token = match check_for_user {
        Ok(_) => {
            user_service.login(test_user).await.unwrap()
        },
        Err(_) => {
            user_service.register_user(test_user).await.unwrap()
        }
    };
    req.header("Authorization", format!("Bearer {}", token))
}

pub async fn make_request(req: test::TestRequest, db: &SqlitePool) -> dev::ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .configure(
                config_app(
                    AppState::as_web_data(db.clone())
                )
            )
    )
    .await;
    test::call_service(&mut app, req.to_request()).await
}

pub async fn get_resp_body<T: DeserializeOwned>(res: dev::ServiceResponse) -> T {
    let result: T = test::read_body_json(res).await;
    result
}