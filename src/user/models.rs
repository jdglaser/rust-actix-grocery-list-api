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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_util;
    use sqlx::sqlite::SqlitePool;

    async fn create_new_user(db: &SqlitePool) -> User {
        let user_template_hashed = UserTemplateHashed {
            username: "JohnDoe".to_string(), 
            hashed_password: "1234".to_string()
        };

        User::create_new_user(&db, &user_template_hashed).await.unwrap()
    }

    #[actix_rt::test]
    async fn it_creates_a_new_user() {
        let db = test_util::setup_test_db().await;

        let user = create_new_user(&db).await;
        assert_eq!("JohnDoe", user.username, "Username does not match");
        assert_eq!("1234", user.password, "Password does not match");
        assert_eq!(1, user.user_id, "User id does not match");
    }

    #[actix_rt::test]
    async fn gets_user() {
        let db = test_util::setup_test_db().await;

        create_new_user(&db).await;

        let result = User::find_user_by_username(&db, "JohnDoe").await;
        match result {
            Ok(user) => {
                assert_eq!("JohnDoe", user.username, "Username does not match");
            },
            Err(err) => {
               panic!("{}", err); 
            }
        }
    }
}