use std::borrow::Borrow;

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result as MongoResult;
use mongodb::options::FindOneOptions;
use mongodb::results::{InsertManyResult, InsertOneResult};
use mongodb::sync::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    value: i64,
}


struct MongoCollection<T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug> {
    collection: Collection<T>,
}

impl<T> MongoCollection<T> where T: DeserializeOwned + Unpin + Send + Sync + Serialize + std::fmt::Debug{
    pub fn new(collection: Collection<T>) -> Self {
        MongoCollection { collection }
    }

    pub fn insert(&self, doc: &T) -> MongoResult<InsertOneResult> {
        self.collection.insert_one(doc, None)
    }

    pub fn insert_many(&self, docs: impl IntoIterator<Item=impl Borrow<T>>) -> MongoResult<InsertManyResult> {
        self.collection.insert_many(docs, None)
    }

    pub fn find_one(&self, filter: Document, sort: &str) -> MongoResult<Option<T>> {
        let options =
            if sort.starts_with('-') {
                FindOneOptions::builder().sort(doc! {sort.trim_start_matches('-'): -1}).build()
            } else if !sort.is_empty() {
                FindOneOptions::builder().sort(doc! {sort: 1}).build()
            } else {
                FindOneOptions::builder().build()
            };
        self.collection.find_one(filter, options)
    }

    pub fn find_many(&self, filter: Document, sort: &str, limit: u64) -> MongoResult<Vec<T>> {
        let mut result: Vec<T> = vec![];
        let cursor = self.collection.find(filter, None)?;
        for d in cursor {
            result.push(d?);
        }
        return Ok(result)
    }
}


fn main() -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Data>("data");


    collection.drop(None)?;

    let d1 = Data { name: "n1".into(), value: 11, id: None };
    let d2 = Data { name: "n2".into(), value: 22, id: None };
    let d3 = Data { name: "n3".into(), value: 33, id: None };


    let data_col = MongoCollection::new(collection);
    let res = data_col.insert(&d1);
    dbg!(res);




    data_col.insert_many(vec!(d2, d3));

    let res = data_col.find_one(doc! {}, "name");
    dbg!(res);

    let res= data_col.find_many(doc!{}, "", 1000);

    Ok(())
}