use actix_web::{http::Method, HttpRequest};

pub struct Request {
    http_request: HttpRequest,
}

impl Request {
    pub fn new(http_request: HttpRequest) -> Self {
        Self { http_request }
    }

    pub fn method(&self) -> &Method {
        self.http_request.method()
    }
}
