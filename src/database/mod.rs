mod film;
mod player;

use actix_web::{HttpResponse, Responder};
use player::Player;
use film::Film;


// GET /
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index page")
}

// GET /player/random
pub async fn get_random_player() -> impl Responder {
    Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
}

// GET /film/random
pub async fn get_random_film() -> impl Responder {
    Film { title: "The Great Dictator", year: 1940 }
}

// GET /player?name=:name
pub async fn get_player() -> impl Responder {
    Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
}

// GET /film?title=:title
pub async fn get_film() -> impl Responder {
    Film { title: "The Great Dictator", year: 1940 }
}