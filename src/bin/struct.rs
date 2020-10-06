use std::error::Error;

use bson::{oid::ObjectId, Document};
use chrono::{DateTime, Utc};
use mongodb::sync::{Client, Collection};
use mongodb::{
    bson::{self, doc, Bson},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};

type DynErr = Box<dyn Error>;

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

impl ToString for DataStatus {
    fn to_string(&self) -> String {
        match self {
            DataStatus::New => String::from("NEW"),
            DataStatus::Ok => String::from("OK"),
            DataStatus::Error => String::from("ERROR"),
        }
    }
}
impl Into<Bson> for DataStatus {
    fn into(self) -> Bson {
        Bson::String(self.to_string())
    }
}

fn insert_data(col: &Collection, name: String, value: i64, status: DataStatus) -> Result<(), DynErr> {
    let new_data = Data {
        id: None,
        name,
        status,
        value,
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

    col.insert_one(bson::to_document(&new_data)?, None)?;
    Ok(())
}

fn find_one(col: &Collection) -> Result<Option<Data>, DynErr> {
    let data = col.find_one(doc! {"name": "n1"}, None)?;
    if data.is_some() {
        Ok(Some(bson::from_document::<Data>(data.unwrap())?))
    } else {
        Ok(None)
    }
}

fn find_many(col: &Collection) -> Result<Vec<Data>, DynErr> {
    println!("\n\nfind_many");
    let mut filter = Document::new();
    filter.insert("status", DataStatus::Ok);
    let options = FindOptions::builder().sort(doc! { "name": -1}).build();
    let res = col.find(filter, options)?.collect::<Result<Vec<_>, _>>()?;
    Ok(res.into_iter().map(|d| bson::from_document(d).unwrap()).collect())
}

fn main() -> Result<(), DynErr> {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let db = client.database("learn__crate__mongodb");
    let col = db.collection("test_struct");

    col.drop(None).unwrap();

    insert_data(&col, String::from("n1"), 10, DataStatus::Ok)?;
    insert_data(&col, String::from("n2"), 13, DataStatus::Error)?;

    let res = find_one(&col)?;
    dbg!(res);

    let res = find_many(&col)?;
    dbg!(res);

    Ok(())
}
