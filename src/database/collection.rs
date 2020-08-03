mod film;
mod player;

use bson::{doc, Document, ordered::OrderedDocument};
use mongodb::{Collection, error:Error};

#[derive(Clone)]
pub struct MongoCollection{
    collection: Collection,
}

impl MongoCollection {
    pub fn new(collection: Collection) -> MongoCollection {
        MongoCollection { collection }
    }

    // pub fn random(&self) -> Result<Option<OrderedDocument>, Error> {
    //     let sample_stage: Document = doc! { "$sample": doc! { "size": 1 } };
    //     let pipeline: Vec<Document> = vec! [sample_stage];
    //     self.collection.aggregate(pipeline)

    // }

    pub fn get(&self, key: &str, value: &str) -> Result<Option<OrderedDocument>, Error> {
        let search_doc: Document = doc! { key: doc! { "$regex": value, "$options": "i" } };
        self.collection.find_one(search_doc, None)
    }
}