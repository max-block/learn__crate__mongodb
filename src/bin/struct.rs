use bson::{oid::ObjectId, ser::Error, Document};
use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc};
use mongodb::sync::{Client, Collection};
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

impl Data {
    fn to_document(&self) -> Result<Document, Error> {
        bson::to_document(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Child {
    name: String,
    group: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum DataStatus {
    NEW,
    OK,
    ERROR,
}

fn insert_data(col: &Collection, name: String, value: i64, status: DataStatus) {
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

    col.insert_one(new_data.to_document().unwrap(), None).unwrap();
}

fn find_one(col: &Collection) {
    let data = col.find_one(doc! {"name": "n1"}, None).unwrap().expect("n1 not found");

    let data: Data = bson::from_bson(data.into()).unwrap();
    println!("{:#?}", data);
}

fn find_many(col: &Collection) {
    let filter = Document::new();
    filter.insert("status", DataStatus::OK);

}

fn main() {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let db = client.database("test");
    let col = db.collection("test");

    col.drop(None).unwrap();

    insert_data(&col, String::from("n1"), 10, DataStatus::OK);
    insert_data(&col, String::from("n2"), 13, DataStatus::ERROR);

    find_one(&col);
}
