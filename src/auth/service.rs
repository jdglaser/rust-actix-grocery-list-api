use actix_web::{FromRequest, Error, dev, HttpRequest};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::http_error::HttpResponseError;
use actix_web::http::StatusCode;
use futures::future::{err, ok, Ready};
use actix_web::error::ErrorUnauthorized;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String
}

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(_) => {
                let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = split[1].trim();
                let key = "MY_SECRET_KEY";
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(key.as_ref()),
                    &Validation::new(Algorithm::HS256)
                ) {
                    Ok(token) => ok(AuthorizationService),
                    Err(error) => err(ErrorUnauthorized("Invalid token"))
                }
            },
            None => err(ErrorUnauthorized("Missing token")),
        }
    }
}