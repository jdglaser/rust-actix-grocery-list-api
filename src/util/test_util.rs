use sqlx::sqlite::SqlitePool;
use actix_web::{test, App, dev, web};
use serde_json::Value;
use futures::StreamExt;

use crate::config_app;
use crate::db;
use crate::db::{DatabaseType};
use crate::user::service::UserService;
use crate::user::{User, UserTemplate};
use crate::state::AppState;

#[allow(dead_code)]
pub async fn setup_test_db() -> SqlitePool {
    let pool = db::get_database_pool(DatabaseType::MEMORY).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    pool
}

#[allow(dead_code)]
pub async fn insert_test_user(db: &SqlitePool) {
    let user_service = UserService::new(db.clone());
    user_service.register_user(UserTemplate::new("JohnDoe".to_string(), "12345".to_string())).await.unwrap();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub async fn get_test_app_state() -> web::Data<AppState> {
    let db = setup_test_db().await;
    AppState::as_web_data(db)
}

#[allow(dead_code)]
pub async fn make_request(mut req: test::TestRequest, db: &SqlitePool) -> dev::ServiceResponse {
    req = login_test_user(req, db).await;
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

#[allow(dead_code)]
pub async fn get_resp_body(mut res: dev::ServiceResponse) -> Value {
    let (bytes, mut _resp) = res.take_body().into_future().await;
    let bytes = bytes.unwrap().unwrap();
    let string = std::str::from_utf8(&bytes);
    let body: Value = serde_json::from_str(string.unwrap()).unwrap();
    body
}