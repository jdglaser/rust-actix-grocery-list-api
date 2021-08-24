use actix_web::{Error, http::StatusCode, error::InternalError};
use std::fmt;

#[allow(non_snake_case)]
pub fn CustomError<T>(err: T, status_code: StatusCode) -> Error
    where
        T: fmt::Debug + fmt::Display + 'static,
{
    InternalError::new(err, status_code).into()
}

