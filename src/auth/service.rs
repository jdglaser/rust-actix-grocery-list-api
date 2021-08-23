use actix_web::{FromRequest, Error, dev, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey, TokenData};
use futures::future::{err, ok, Ready};
use actix_web::error::ErrorUnauthorized;
use crate::auth::models::Claims;
use chrono::{Utc, Duration};
use crate::config::get_config;

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(_) => {
                let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = split[1].trim();
                match AuthorizationService::decode_token(token) {
                    Ok(_) => ok(AuthorizationService),
                    Err(_) => err(ErrorUnauthorized("Invalid token"))
                }
            },
            None => err(ErrorUnauthorized("Missing token")),
        }
    }
}

impl AuthorizationService {
    pub fn encode_token(username: &str) -> String {
        let config = get_config();
        let key = config.secret_key;

        let my_claims = Claims {
            sub: username.to_string(),
            exp: (Utc::now() + Duration::days(365)).timestamp() as usize,
        };

        let token = encode::<Claims>(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key.as_bytes())
        ).unwrap();

        token
    }

    pub fn decode_token(token: &str) -> Result<TokenData<Claims>, Error> {
        let config = get_config();
        let key = config.secret_key;

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_ref()),
            &Validation::new(Algorithm::HS256)
        ) {
            Ok(token) => Ok(token),
            Err(_) => Err(ErrorUnauthorized("Invalid token"))
        }
    }
}