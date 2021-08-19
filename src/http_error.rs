use actix_web::{http, ResponseError, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponseError {
    detail: String,
    status_code: u16
}

impl HttpResponseError {
    pub fn new(detail: String, status_code: http::StatusCode) -> HttpResponseError {
        HttpResponseError {
            detail,
            status_code: status_code.as_u16()
        }
    }
}

impl std::fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.detail)
    }
}

impl ResponseError for HttpResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header(http::header::CONTENT_TYPE, "application/problem+json")
            .json(self)
    }

    fn status_code(&self) -> http::StatusCode {
        if let Ok(status_code) = http::StatusCode::from_u16(self.status_code) {
            return status_code
        } else {
            panic!("Invalid status code");
        }
    }
}