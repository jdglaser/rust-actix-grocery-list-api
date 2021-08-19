use actix_web::{HttpServer, App, web};

mod item;
mod db;
mod http_error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let db = db::Database::new();

        App::new()
            .data(db)
            .service(chat::hello)
            .service(chat::echo)
            .configure(item::init)
            .route("/hey", web::get().to(chat::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
