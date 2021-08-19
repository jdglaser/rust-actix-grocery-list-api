use actix_web::{get, post, Responder, HttpResponse, web, http};
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::http_error::HttpResponseError;

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: usize,
    name: String,
    category: String,
    checked: bool
}

impl Item {
    pub fn new(id: usize, new_item: ItemTemplate) -> Item {
        Item {
            id,
            name: new_item.name,
            category: new_item.category,
            checked: false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemTemplate {
    name: String,
    category: String
}


#[get("/{id}")]
async fn get_item(db: web::Data<Database>, id: web::Path<usize>) -> impl Responder {
    let db = db.items.lock().unwrap();
    
    let result = db.iter().find(|i| i.id == *id.as_ref());

    match result {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        _ => Err(HttpResponseError::new(
            format!("Item {} not found", id.into_inner()),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("")]
async fn get_items() -> impl Responder {
    HttpResponse::Ok().body("All items")
}

#[post("")]
async fn create_item(db: web::Data<Database>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {
    
    let mut items = db.items.lock().unwrap();
    let size = items.len();

    items.push(Item::new(size, new_item.into_inner()));
    HttpResponse::Ok().body("Created item")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/item")
        .service(get_item)
        .service(get_items)
        .service(create_item)
    );
}

