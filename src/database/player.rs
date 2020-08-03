use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

// "_id" is useful if it's going to be used to upload / update documents
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "primaryName")]
    pub name: Option<String>,
    #[serde(rename = "birthYear")]
    pub birth_year: Option<i32>,
    #[serde(rename = "deathYear")]
    pub death_year: Option<i32>,
}

impl Responder for Player {
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