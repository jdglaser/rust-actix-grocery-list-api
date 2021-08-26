use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::Row;
use sqlx::sqlite::{SqliteRow};
use anyhow::Result;
use futures::TryStreamExt;

use crate::user::User;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTemplate {
    pub name: String,
    pub category: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    item_id: i32,
    user: User,
    name: String,
    category: String,
    is_checked: bool
}

impl Item {
    pub async fn get_item(db: &SqlitePool, id: i32) -> Result<Item> {        
        let result = sqlx::query("SELECT * FROM item WHERE item_id = ?")
            .bind(id)
            .fetch_one(db)
            .await?;
        
        let item = Item::map_item(result, db).await?;
        
        Ok(item)
    }

    pub async fn get_items(db: &SqlitePool) -> Result<Vec<Item>> {
        let mut result = sqlx::query("SELECT * FROM item").fetch(db);

        let mut items = Vec::new();
        while let Some(row) = result.try_next().await? {
            // map the row into a user-defined domain type
            items.push(Item::map_item(row, db).await?)
        }

        Ok(items)
    }

    pub async fn create_item(db: &SqlitePool, new_item: ItemTemplate, username: &str) -> Result<Item> {

        let user = User::find_user_by_username(db, username).await?;
        let result = sqlx::query("INSERT INTO item (name, category, user_id) VALUES (?, ?, ?) RETURNING *")
            .bind(new_item.name)
            .bind(new_item.category)
            .bind(user.user_id)
            .fetch_one(db)
            .await?;

        let item = Item::map_item(result, db).await?;

        Ok(item)
    }

    pub async fn map_item(row: SqliteRow, db: &SqlitePool) -> Result<Item> {
        let user_id = row.get("user_id");
        let user = User::find_user_by_id(db, user_id).await?;
        Ok(Item {
            item_id: row.get("item_id"),
            name: row.get("name"),
            user,
            category: row.get("category"),
            is_checked: row.get("is_checked")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_util;

    async fn create_test_item(pool: &SqlitePool) -> Item {
        let result = Item::create_item(pool, ItemTemplate {
            name: String::from("foo"),
            category: String::from("bar")
        }, "JohnDoe").await.unwrap();
        result
    }

    #[actix_rt::test]
    async fn it_creates_an_item() {
        let pool = test_util::setup_test_db().await;
        test_util::insert_test_user(&pool).await;

        let item = create_test_item(&pool).await;

        assert_eq!("foo", item.name)
    }

    #[actix_rt::test]
    async fn it_gets_item() {
        let pool = test_util::setup_test_db().await;
        test_util::insert_test_user(&pool).await;

        let item = create_test_item(&pool).await;

        let result = Item::get_item(&pool, item.item_id).await;
        assert!(result.is_ok());
        assert_eq!(item.item_id, result.unwrap().item_id);
    }

    #[actix_rt::test]
    async fn it_gets_all_items() {
        let pool = test_util::setup_test_db().await;
        test_util::insert_test_user(&pool).await;

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