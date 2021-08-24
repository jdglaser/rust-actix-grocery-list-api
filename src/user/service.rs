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
                Err(CustomError(format!("User {} not found", &user_template.username), StatusCode::NOT_FOUND))
            }
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

