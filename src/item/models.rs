use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::Row;
use sqlx::sqlite::{SqliteRow};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTemplate {
    name: String,
    category: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    item_id: i32,
    name: String,
    category: String,
    is_checked: bool
}

impl Item {
    pub async fn get_item(db: &SqlitePool, id: i32) -> Result<Item> {        
        let result = sqlx::query("SELECT * FROM item WHERE item_id = ?")
            .bind(id)
            .map(Item::map_item)
            .fetch_one(db)
            .await?;
        
        Ok(result)
    }

    pub async fn get_items(db: &SqlitePool) -> Result<Vec<Item>> {
        let result = sqlx::query("SELECT * FROM item").map(Item::map_item).fetch_all(db).await?;

        Ok(result)
    }

    pub async fn create_item(db: &SqlitePool, new_item: ItemTemplate) -> Result<Item> {

        let result = sqlx::query("INSERT INTO item (name, category) VALUES (?, ?) RETURNING *")
            .bind(new_item.name)
            .bind(new_item.category)
            .map(Item::map_item)
            .fetch_one(db)
            .await?;

        Ok(result)
    }

    pub fn map_item(row: SqliteRow) -> Item {
        Item {
            item_id: row.get("item_id"),
            name: row.get("name"),
            category: row.get("category"),
            is_checked: row.get("is_checked")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::get_database_pool;

    async fn setup_db() -> SqlitePool {
        let pool = get_database_pool().await;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .unwrap();
        
        pool
    }

    async fn create_test_item(pool: &SqlitePool) -> Item {
        let result = Item::create_item(pool, ItemTemplate {
            name: String::from("foo"),
            category: String::from("bar")
        }).await.unwrap();
        result
    }

    #[actix_rt::test]
    async fn it_creates_an_item() {
        let pool = setup_db().await;
        let item = create_test_item(&pool).await;

        assert_eq!("foo", item.name)
    }

    #[actix_rt::test]
    async fn it_gets_item() {
        let pool = setup_db().await;
        let item = create_test_item(&pool).await;

        let result = Item::get_item(&pool, item.item_id).await;
        assert!(result.is_ok());
        assert_eq!(item.item_id, result.unwrap().item_id);
    }

    #[actix_rt::test]
    async fn it_gets_all_items() {
        let pool = setup_db().await;
        let _ = create_test_item(&pool).await;
        let _ = create_test_item(&pool).await;

        let result = Item::get_items(&pool).await;
        assert!(result.is_ok());
        let items = result.unwrap();
        println!("{:?}", items);
        assert_eq!(2, items.len());
        assert_eq!("foo", items[0].name);
    }
}