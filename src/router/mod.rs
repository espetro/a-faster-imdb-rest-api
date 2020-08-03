use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct Film {
    title: String,
    year: u32,
}

impl Responder for Film {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(
            HttpResponse::Ok().content_type("application/json").body(body)
        ))
    }
}

#[derive(Serialize)]
struct Player {
    name: String,
    birth_year: u32,
    death_year: u32,
}

impl Responder for Player {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(
            HttpResponse::Ok().content_type("application/json").body(body)
        ))
    }
}

// GET /
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index page")
}

// GET /player/random
pub async fn get_random_player() -> impl Responder {
    Player { name: "Charles Chaplin".to_string(), birth_year: 1889, death_year: 1977 }
}

// GET /film/random
pub async fn get_random_film() -> impl Responder {
    Film { title: "The Great Dictator".to_string(), year: 1940 }
}

// GET /player?name=:name
pub async fn get_player() -> impl Responder {
    Player { name: "Charles Chaplin".to_string(), birth_year: 1889, death_year: 1977 }
}

// GET /film?title=:title
pub async fn get_film() -> impl Responder {
    Film { title: "The Great Dictator".to_string(), year: 1940 }
}