use actix_web::{HttpServer, App, web, middleware::Logger};

#[macro_use]
extern crate log;

mod routes;
mod db;
mod http_error;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting app!");

    let thread = std::thread::current().id();
    let db = web::Data::new(db::Database::new(&format!("{:?}", thread)));

    HttpServer::new(move || {

        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db.clone())
            .service(chat::hello)
            .service(chat::echo)
            .configure(routes::init)
            .route("/hey", web::get().to(chat::manual_hello))
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
