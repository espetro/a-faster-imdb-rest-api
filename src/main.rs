mod database;

use std::{io::Result, env};
use mongodb::{options::ClientOptions, Client};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use database::{DBState, get_random_film, get_random_player, get_film, get_player};

// GET /
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index page")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let conn = client.database("imdb");

    let state = DBState {
        film_coll: conn.collection("films"),
        crew_coll: conn.collection("crew"),
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(index))
            .route("/player/random", web::get().to(get_random_player))
            .route("/film/random", web::get().to(get_random_film))
            .route("/player", web::get().to(get_player))
            .route("/film", web::get().to(get_film))
    })
    .bind("localhost:8080")?
    .run()
    .await

}
