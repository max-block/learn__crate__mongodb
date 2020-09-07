use std::error::Error;

use chrono::Utc;
use mongodb::bson::{doc, Document};
use mongodb::sync::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("test");
    let col = db.collection("test");

    col.drop(None)?;

    let docs = vec![
        doc! {"name": "n1", "value":1, "created_at": Utc::now()},
        doc! {"name": "n2", "value":2, "created_at": Utc::now()},
    ];
    col.insert_many(docs, None)?;

    let n1: Document = col.find_one(doc! {"name": "n1"}, None)?.unwrap();


    println!("n1.created_at: {}", n1.get_datetime("created_at").unwrap());

    Ok(())
}
