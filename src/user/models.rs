use sqlx::SqlitePool;
use sqlx::{Row, sqlite::SqliteRow};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTemplate {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTemplateHashed {
    pub username: String,
    pub hashed_password: String,
}

impl User {
    pub async fn find_user_by_username(db: &SqlitePool, username: &str) -> Result<User> {
        let result = sqlx::query("SELECT * FROM user WHERE username = ?")
            .bind(username)
            .map(User::map_user)
            .fetch_one(db)
            .await?;

        Ok(result)
    }

    pub async fn create_new_user(db: &SqlitePool, user_template: &UserTemplateHashed) -> Result<User> {
        let result = sqlx::query("INSERT INTO user (username, password) VALUES (?, ?) RETURNING *")
            .bind(&user_template.username)
            .bind(&user_template.hashed_password)
            .map(User::map_user)
            .fetch_one(db)
            .await?;
        
        Ok(result)
    }

    pub fn map_user(row: SqliteRow) -> User {
        User {
            user_id: row.get("user_id"),
            username: row.get("username"),
            password: row.get("password"),
        }
    }
}

impl UserTemplate {
    pub fn new(username: String, password: String) -> UserTemplate {
        UserTemplate {
            username,
            password
        }
    }
}