use std::error::Error;


use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc};
use mongodb::error::Error as MongoError;
use mongodb::results::InsertOneResult;
use mongodb::sync::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    value: i64,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}

fn insert_data(col: &Collection) -> Result<InsertOneResult, MongoError> {
    let new_data = Data {
        id: None,
        name: "n1".to_string(),
        value: 10,
        tags: vec!["a1".to_string(), "b2".to_string()],
        created_at: Utc::now(),
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

    insert_data(&col)?;

    let one_data = find_one(&col);
    println!("{:?}", one_data);
    Ok(())
}
