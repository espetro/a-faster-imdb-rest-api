use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Film {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "primaryTitle")]
    pub primary_title: Option<String>,
    #[serde(rename = "genres")]
    pub genres: Option<String>,
    #[serde(rename = "startYear")]
    pub start_year: Option<i32>,
    #[serde(rename = "endYear")]
    pub end_year: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
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