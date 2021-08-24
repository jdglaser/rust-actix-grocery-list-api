use actix_web::{HttpServer, App, web, middleware::Logger, HttpResponse, error};

#[macro_use]
extern crate log;

use serde::{Deserialize, Serialize};

mod item;
mod config;
mod auth;

mod util;
mod user;

use util::db;
use util::state;

use crate::config::get_config;

#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String
}

pub fn config_app(app_state: web::Data<state::AppState>) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(app_state.clone());
        cfg.route("/health", web::get().to(|| HttpResponse::Ok().json(HealthStatus{status: "Ok".to_string()})));
        cfg.app_data(
            web::JsonConfig::default().error_handler(
                |err, _req| {
                     error::InternalError::from_response(
                          "",
                          HttpResponse::BadRequest()
                              .content_type("application/json")
                              .body(format!(r#"{{"error":"{}"}}"#, err)),
                      )
                      .into()
                  }
            )
        );
        item::init(cfg);
        user::init(cfg);
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_type = if get_config().database_type == "file" {
        let _ = std::fs::create_dir("./data/");
        let conn = rusqlite::Connection::open("./data/database.db").unwrap();
        let _ = conn.close();
        db::DatabaseType::FILE
    } else {
        db::DatabaseType::MEMORY
    };

    info!("Starting app!");

    let database_pool = db::get_database_pool(database_type).await;

    sqlx::migrate!("./migrations")
        .run(&database_pool)
        .await
        .unwrap();

    let app_state = web::Data::new(state::AppState::new(database_pool));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(config_app(app_state.clone()))
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
