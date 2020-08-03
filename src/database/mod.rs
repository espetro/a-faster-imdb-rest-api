mod film;
mod player;

use actix_web::{web, HttpResponse, HttpRequest, Responder};
use mongodb::{Collection, bson::doc};
use player::Player;
use film::Film;

#[derive(Clone)]
pub struct DBState {
    pub film_coll: Collection,
    pub crew_coll: Collection,
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
pub async fn get_player(app_data: web::Data<DBState>, req: HttpRequest) -> impl Responder {
    // Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
    // let result = web::block(move || app_data.crewColl.find_one(doc! {}, None)).await;
    let result = &app_data.crew_coll.find_one(doc! {}, None).await;

    match result {
        Ok(doc) => {
            let body = serde_json::to_string(&doc).unwrap();
            return HttpResponse::Ok().content_type("application/json").body(body);
        },
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        },
    }
}

// GET /film?title=:title
pub async fn get_film() -> impl Responder {
    Film { title: "The Great Dictator", year: 1940 }
}