use actix_web::{HttpServer, App, web, middleware::Logger};
use rusqlite::Connection;

#[macro_use]
extern crate log;

mod routes;
mod db;
mod http_error;
mod model;

mod tests;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting app!");

    let conn = Connection::open("data.db").unwrap();
    conn.close().unwrap();
    let app_state = web::Data::new(state::AppState::new().await);
    
    sqlx::migrate!("./migrations")
        .run(&app_state.database_pool)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .configure(routes::init)
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
