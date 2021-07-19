use mongodb::sync::Client;
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
    let collection = database.collection::<Book>("books");


    let data_list = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "Animal Farm".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

// Insert some books into the "mydb.books" collection.
    collection.insert_many(docs, None)?;
    Ok(())
}