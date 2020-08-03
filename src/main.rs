// mod db;
mod router;

use std::io::Result;
use actix_web::{web, App, HttpServer};
use router::{index, get_film, get_player, get_random_film, get_random_player};

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
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
