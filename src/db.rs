use crate::item;
use std::sync::Mutex;

pub struct Database {
    pub items: Mutex<Vec<item::Item>>
}

impl Database {
    pub fn new () -> Self {
        Self {
            items: Mutex::from(Vec::new())
        }
    }
}