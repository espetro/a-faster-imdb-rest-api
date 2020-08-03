mod film;
mod player;

use actix_web::{web, HttpResponse, Responder};
use mongodb::{Collection, bson::doc};
use serde::Deserialize;
// use player::Player;
// use film::Film;

#[derive(Clone)]
pub struct DBState {
    pub film_coll: Collection,
    pub crew_coll: Collection,
}

#[derive(Deserialize)]
pub struct Info {
    name: Option<String>,
    title: Option<String>,
}

// GET /player/random
pub async fn get_random_player(app_data: web::Data<DBState>) -> impl Responder {
    // Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
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

// GET /film/random
pub async fn get_random_film(app_data: web::Data<DBState>) -> impl Responder {
    // Film { title: "The Great Dictator", year: 1940 }
    // let db_query = doc! { "$sample": doc! { "size": 1 } };
    let result = &app_data.film_coll.find_one(doc! {}, None).await;

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

// GET /player?name=:name
pub async fn get_player(app_data: web::Data<DBState>, info: web::Query<Info>) -> impl Responder {
    // Player { name: "Charles Chaplin", birth_year: 1889, death_year: 1977 }
    let name: String;

    match &info.name {
        Some(query) => {
            name = query.to_string()
        },
        None => {
            return HttpResponse::BadRequest().finish();
        }
    }

    println!("Asking for player: {}", name);
    let db_query = doc! { "primaryName": doc! { "$regex": name, "$options": "i" } };
    let result = &app_data.crew_coll.find_one(db_query, None).await;

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
pub async fn get_film(app_data: web::Data<DBState>, info: web::Query<Info>) -> impl Responder {
    // Film { title: "The Great Dictator", year: 1940 }
    let title: String;

    match &info.title {
        Some(query) => {
            title = query.to_string()
        },
        None => {
            return HttpResponse::BadRequest().finish();
        }
    }

    println!("Asking for title: {}", title);

    let db_query = doc! { "primaryTitle": doc! { "$regex": title, "$options": "i" } };
    let result = &app_data.film_coll.find_one(db_query, None).await;

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