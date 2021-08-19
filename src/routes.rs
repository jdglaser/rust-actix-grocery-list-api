use actix_web::{get, post, Responder, HttpResponse, web, http};

use crate::state;
use crate::http_error::HttpResponseError;

use crate::model::{Item, ItemTemplate};


#[get("/{id}")]
async fn get_item(state: web::Data<state::AppState>, id: web::Path<i32>) -> impl Responder {
    let result = Item::get_item(&state.database_pool, *id).await;

    info!("{:?}", result);

    match result {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        _ => Err(HttpResponseError::new(
            format!("Item {} not found", id),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("/")]
async fn get_items(state: web::Data<state::AppState>) -> impl Responder {
    let items = Item::get_items(&state.database_pool).await;

    if let Ok(items) = items {
        Ok(HttpResponse::Ok().json(items))
    } else {
        Err(HttpResponseError::new(String::from("error retrieving items"), http::StatusCode::BAD_REQUEST))
    }
}

#[post("/")]
async fn create_item(state: web::Data<state::AppState>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {

    let created_item = Item::create_item(&state.database_pool, new_item.into_inner()).await;

    if let Ok(created_item) = created_item {
        Ok(HttpResponse::Ok().json(created_item))
    } else {
        Err(HttpResponseError::new(String::from("Error creating item"), http::StatusCode::BAD_REQUEST))
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

