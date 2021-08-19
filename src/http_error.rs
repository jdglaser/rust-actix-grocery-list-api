use actix_web::{http, ResponseError, HttpResponse, dev::HttpResponseBuilder};

#[derive(Debug)]
pub struct HttpResponseError {
    message: String,
    status_code:  http::StatusCode
}

impl HttpResponseError {
    pub fn new(message: String, status_code: http::StatusCode) -> HttpResponseError {
        HttpResponseError {
            message,
            status_code
        }
    }
}

impl std::fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl ResponseError for HttpResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        self.status_code
    }
}