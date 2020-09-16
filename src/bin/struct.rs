use std::error::Error;

use chrono::Utc;
use mongodb::bson::{doc, Document};
use mongodb::error::Error as MongoError;
use mongodb::results::InsertOneResult;
use mongodb::sync::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    name: String,
    value: i64,
}

fn insert_data(col: &Collection) -> Result<InsertOneResult, MongoError> {
    let new_data = Data {
        id: None,
        name: "n1".to_string(),
        value: 10,
    };
    let new_data = bson::to_bson(&new_data)?;
    col.insert_one(new_data.as_document().unwrap().to_owned(), None)
}

fn find_one(col: &Collection) -> Result<Data, MongoError> {
    let data = col
        .find_one(doc! {"name": "n1"}, None)?
        .expect("n1 not found");

    let data: Data = bson::from_bson(data.into())?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("test");
    let col = db.collection("test");

    col.drop(None)?;

    insert_data(&col);

    let one_data = find_one(&col);
    println!("{:?}", one_data);
    Ok(())
}
