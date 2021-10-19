use mongodb::bson::{doc, Document};
use mongodb::sync::Client;

fn main() -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Document>("data");
    collection.drop(None)?;

    collection.insert_one(doc!{"name": "n1", "value": 1}, None)?;
    collection.insert_many(vec![doc!{"name": "n2", "value": 2}], None)?;

    // let all: Vec<Document> = collection.find(doc!{}, None)?.collect();
    let all: Vec<mongodb::error::Result<Document>> = collection.find(doc!{}, None)?.collect();
    dbg!(all);

    
    

    Ok(())
}
