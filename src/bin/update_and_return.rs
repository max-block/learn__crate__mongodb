use std::error::Error;

use mongodb::{bson::doc, options::FindOneAndUpdateOptions, options::ReturnDocument, sync::Client};

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("test");
    let col = db.collection("test");

    col.drop(None)?;

    col.insert_one(doc! {"name": "n1", "value": 1}, None)?;

    let before_update = col.find_one(doc! {"name": "n1"}, None)?;
    dbg!(before_update);

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let after_update = col.find_one_and_update(doc! {"name": "n1"}, doc! {"$set": {"value": 2}}, options)?;
    dbg!(after_update);

    Ok(())
}
