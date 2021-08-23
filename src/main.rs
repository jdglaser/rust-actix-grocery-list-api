use actix_web::{HttpServer, App, web, middleware::Logger};

#[macro_use]
extern crate log;

mod db;
mod item;
mod config;
mod auth;

mod state;
mod errors;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting app!");

    let database_pool = db::get_database_pool().await;

    sqlx::migrate!("./migrations")
        .run(&database_pool)
        .await
        .unwrap();

    let app_state = web::Data::new(state::AppState::new(database_pool).await);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .configure(item::init)
            .configure(user::init)
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
