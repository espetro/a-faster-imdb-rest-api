mod database;

use std::{io::Result, env, sync::*};
use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use database::{index, get_film, get_player, get_random_film, get_random_player};

#[actix_rt::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("LocalAPI".to_string());

    let _client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    // Print the databases in our MongoDB cluster:
    let another_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = mongodb::Client::with_options(another_options).unwrap();
    println!("Databases:");
    match client.list_database_names(None, None).await {
        Ok(names) => println!("{:?}", names),
        Err(_) => { println!("Got an error") }
    }


    HttpServer::new(move || {
        App::new()
            .app_data(_client.clone())
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
