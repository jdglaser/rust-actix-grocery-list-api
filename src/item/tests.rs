#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use crate::config_app;
    use crate::db;

    #[actix_rt::test]
    async fn test_get_item() {
        std::env::set_var("DATABASE_TYPE", "memory");
        let database_pool = db::get_database_pool().await;

        sqlx::migrate!("./migrations")
            .run(&database_pool)
            .await
            .unwrap();

        let app_state = web::Data::new(crate::state::AppState::new(database_pool).await);

        let mut app = test::init_service(App::new().configure(config_app(app_state))).await;
        let req = test::TestRequest::with_uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}