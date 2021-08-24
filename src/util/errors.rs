use actix_web::{Error, http::StatusCode, error::InternalError, HttpResponse, dev::HttpResponseBuilder};
use std::fmt;

#[allow(non_snake_case)]
pub fn CustomError<T>(err: T, status_code: StatusCode) -> Error
    where
        T: fmt::Debug + fmt::Display + 'static,
{
    InternalError::new(err, status_code).into()
}

#[allow(non_snake_case)]
pub fn HttpResponseError<T>(err: T, status_code: StatusCode) -> HttpResponse
    where
        T: fmt::Debug + fmt::Display + 'static,
{
    HttpResponseBuilder::from(HttpResponse::from_error(CustomError(err, status_code)))
        .content_type("application/json")
        .finish()
}

