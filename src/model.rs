// I guess we can just put the model and repo code together
use serde::{Deserialize, Serialize};
use crate::db::Database;
use actix_web::web;

#[derive(Serialize, Deserialize)]
pub struct ItemTemplate {
    name: String,
    category: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub async fn get_item(db: &web::Data<Database>, id: usize) -> Option<Item> {        
        let items = db.items.lock().unwrap();
        let result = items.iter().find(|i| i.id == id);
    
        if let Some(item) = result {
            Some(item.clone())
        } else {
            None
        }
    }

    pub async fn get_items(db: &web::Data<Database>) -> Vec<Item> {
        db.items.lock().unwrap().to_vec()
    }

    pub async fn create_item(db: &web::Data<Database>, new_item: ItemTemplate) {

        let mut items = db.items.lock().unwrap();
        let size = items.len();

        let item = Item::new(size, new_item);

        items.push(item);
    }
}