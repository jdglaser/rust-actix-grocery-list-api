use actix_web::{get, post, Responder, HttpResponse, web, http};

use crate::state;

use crate::item::{Item, ItemTemplate};
use crate::auth::service::AuthorizationService;
use crate::errors::CustomError;


#[get("/{id}")]
async fn get_item(_: AuthorizationService, state: web::Data<state::AppState>, id: web::Path<i32>) -> impl Responder {
    let result = Item::get_item(&state.database_pool, *id).await;

    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(error) => Err(CustomError(
            format!("Item {} not found", id),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("/")]
async fn get_items(_: AuthorizationService, state: web::Data<state::AppState>) -> impl Responder {
    let result = Item::get_items(&state.database_pool).await;

    match result {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(error) => {
            error!("Error: {}", error.to_string());
            Err(CustomError("Problem getting items from database", http::StatusCode::BAD_REQUEST))
        }
    }
}

#[post("/")]
async fn create_item(_: AuthorizationService,
                     state: web::Data<state::AppState>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {

    let created_item = Item::create_item(&state.database_pool, new_item.into_inner()).await;

    if let Ok(created_item) = created_item {
        Ok(HttpResponse::Ok().json(created_item))
    } else {
        Err(CustomError("Error creating item", http::StatusCode::BAD_REQUEST))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/item")
            .service(get_item)
            .service(get_items)
            .service(create_item)
    );
}

