use crate::model::Item;
use std::sync::Mutex;

pub struct Database {
    pub items: Mutex<Vec<Item>>
}

impl Database {
    pub fn new (thread_name: &str) -> Self {

        info!("Creating new db on thread {}", thread_name);
        Self {
            items: Mutex::from(Vec::new())
        }
    }
}