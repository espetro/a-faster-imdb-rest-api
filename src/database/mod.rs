mod film;
mod player;

use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client}; // , options::FindOptions};
// use mongodb::bson::{doc, Bson};
use std::sync::Mutex;
use player::Player;
use film::Film;

// pub struct Database {
//     pub client: Client,
// }

// impl Database {
//     async fn new(&self) -> Database {
//         let result = Client::with_uri_str("mongodb://localhost:27017").await;

//         match result {
//             Ok(conn) => Database { client: conn },
//             Err(_) => None,
//         }
        
//     }
// }

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
pub async fn get_player(_data: web::Data<Mutex<Client>>) -> impl Responder {
    Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
}

// GET /film?title=:title
pub async fn get_film() -> impl Responder {
    Film { title: "The Great Dictator", year: 1940 }
}