use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId},
    IndexModel,
};
use mongodb::{options::IndexOptions, sync::Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    status: DataStatus,
    value: i64,
    tags: Vec<String>,
    children: Vec<Child>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Child {
    name: String,
    group: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum DataStatus {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ERROR")]
    Error,
}

fn main() -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Data>("data");
    collection.drop(None)?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        None,
    )?;

    let new_data = Data {
        id: None,
        name: "n1".to_string(),
        status: DataStatus::Ok,
        value: 17,
        tags: vec!["a1".to_string(), "b2".to_string()],
        children: vec![
            Child {
                name: "c1".to_string(),
                group: "g1".to_string(),
            },
            Child {
                name: "c2".to_string(),
                group: "g2".to_string(),
            },
        ],
        created_at: Utc::now(),
    };
    collection.insert_one(new_data, None).unwrap();

    // search a doc
    let r = collection.find_one(doc! {"name": "n1"}, None);
    dbg!(r.unwrap());

    Ok(())
}
