use actix_web::{get, post, Responder, HttpResponse, web, http};

use crate::db::Database;
use crate::http_error::HttpResponseError;

use crate::model::{Item, ItemTemplate};


#[get("/{id}")]
async fn get_item(db: web::Data<Database>, id: web::Path<usize>) -> impl Responder {
    let result = Item::get_item(&db, *id).await;

    match result {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        _ => Err(HttpResponseError::new(
            format!("Item {} not found", id.into_inner()),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("")]
async fn get_items(db: web::Data<Database>) -> impl Responder {
    let items = Item::get_items(&db).await;

    HttpResponse::Ok().json(items)
}

#[post("")]
async fn create_item(db: web::Data<Database>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {

    Item::create_item(&db, new_item.into_inner()).await;
    HttpResponse::Ok().body("Created item")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/item")
        .service(get_item)
        .service(get_items)
        .service(create_item)
    );
}

