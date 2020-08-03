use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
pub struct Film {
    pub title: &'static str,
    pub year: u32,
}

impl Responder for Film {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // JSONify a serializable struct to String, then unwrap to get a &str
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(
            HttpResponse::Ok().content_type("application/json").body(body)
        ))
    }
}