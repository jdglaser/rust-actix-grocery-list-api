use sqlx::SqlitePool;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use anyhow::{Result};
use actix_web::{Error, http::StatusCode};

use crate::auth::AuthorizationService;
use crate::util::errors::CustomError;
use crate::user::models::{User, UserTemplate, UserTemplateHashed};

pub struct UserService {
    pub db: SqlitePool
}

impl UserService {
    pub fn new(db: SqlitePool) -> UserService {
        UserService {
            db
        }
    }

    pub async fn register_user(&self, user_template: UserTemplate) -> Result<String, Error> {
        let checked_user = User::find_user_by_username(&self.db, &user_template.username).await;
        match checked_user {
            Ok(_) => Err(CustomError(format!("User with username {} already exists", &user_template.username),
                                     StatusCode::CONFLICT)),
            Err(_) => {
                let user_template_hashed = UserService::get_user_template_hashed(user_template);
                match User::create_new_user(&self.db, &user_template_hashed).await {
                    Ok(user) => {
                        let token = AuthorizationService::encode_token(&user.username);
                        Ok(token)
                    },
                    Err(error) => {
                        error!("Error: {}", error);
                        Err(CustomError(format!("Failed to create user {}", &user_template_hashed.username),
                                        StatusCode::INTERNAL_SERVER_ERROR))
                    }
                }
            }
        }
    }

    pub async fn login(&self, user_template: UserTemplate) -> Result<String, Error> {
        match User::find_user_by_username(&self.db, &user_template.username).await {
            Ok(user) => {
                let user_template_hashed = UserService::get_user_template_hashed(user_template);
                if user_template_hashed.hashed_password == user.password {
                    let token = AuthorizationService::encode_token(&user.username);
                    Ok(token)
                } else {
                    Err(CustomError("Invalid login credentials", StatusCode::UNAUTHORIZED))
                }
            },
            Err(_) => {
                Err(CustomError(format!("User {} not found", user_template.username), StatusCode::NOT_FOUND))
            }
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, Error> {
        match User::find_user_by_username(&self.db, &username).await {
            Ok(user) => Ok(user),
            Err(_) => Err(CustomError(format!("User {} not found", username), StatusCode::NOT_FOUND))
        }
    }

    fn get_user_template_hashed(user_template: UserTemplate) -> UserTemplateHashed {
        let mut sha = Sha256::new();
        sha.input_str(&user_template.password);
        UserTemplateHashed {
            username: user_template.username,
            hashed_password: sha.result_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_util;
    use crate::user::UserTemplate;

    struct TestState {
        db: SqlitePool,
        user_service: UserService,
    }

    async fn setup() -> TestState {
        let db = test_util::setup_test_db().await;
        TestState {
            db: db.clone(),
            user_service: UserService::new(db.clone()),
        }
    }

    #[actix_rt::test]
    async fn it_registers_user() {
        let test_state = setup().await;

        let user_template = UserTemplate::new("JohnDoe".to_string(), "1234".to_string());

        let result = test_state.user_service.register_user(user_template).await;
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    async fn it_logs_in_user() {
        let TestState {
            user_service,
            db: _db
        } = setup().await;

        let user_template = UserTemplate::new("JohnDoe".to_string(), "1234".to_string());

        let _ = user_service.register_user(user_template.clone()).await;
        let result = user_service.login(user_template.clone()).await;

        assert!(result.is_ok());

        let user = user_service.get_user_by_username(&user_template.username).await.unwrap();

        let user_template_hashed = UserService::get_user_template_hashed(user_template);
        assert_eq!(user_template_hashed.hashed_password, user.password);
    }
}

