use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::Row;
use sqlx::sqlite::{SqliteRow};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTemplate {
    name: String,
    cool_category: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    id: i32,
    name: String,
    category: String,
    is_checked: bool
}

impl Item {
    pub async fn get_item(db: &SqlitePool, id: i32) -> Result<Item> {        
        let result = sqlx::query("SELECT * FROM items WHERE id = ?")
            .bind(id)
            .map(Item::map_item)
            .fetch_one(db)
            .await?;
        
        Ok(result)
    }

    pub async fn get_items(db: &SqlitePool) -> Result<Vec<Item>> {
        let result = sqlx::query("SELECT * FROM items").map(Item::map_item).fetch_all(db).await?;

        Ok(result)
    }

    pub async fn create_item(db: &SqlitePool, new_item: ItemTemplate) -> Result<Item> {

        let result = sqlx::query("INSERT INTO items (name, category) VALUES (?, ?) RETURNING *")
            .bind(new_item.name)
            .bind(new_item.cool_category)
            .map(Item::map_item)
            .fetch_one(db)
            .await?;

        Ok(result)
    }

    pub fn map_item(row: SqliteRow) -> Item {
        Item {
            id: row.get("id"),
            name: row.get("name"),
            category: row.get("category"),
            is_checked: row.get("is_checked")
        }
    }
}