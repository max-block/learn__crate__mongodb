use chrono::Utc;
use mongodb::{error::Error, options::FindOptions};
use mongodb::sync::Client;
use mongodb::{
    bson::{doc, Document},
    sync::Collection,
};

fn main() -> Result<(), Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("test");
    let col = db.collection("test");

    col.drop(None)?;

    insert_data(&col)?;
    find_one(&col)?;
    find_many(&col)?;

    Ok(())
}

fn insert_data(col: &Collection) -> Result<(), Error> {
    let docs = vec![
        doc! {"name": "n1", "value":1, "created_at": Utc::now()},
        doc! {"name": "n2", "value":2, "created_at": Utc::now()},
    ];
    col.insert_many(docs, None)?;
    Ok(())
}

fn find_one(col: &Collection) -> Result<(), Error> {
    let res = col.find_one(doc! {"name": "n1"}, None)?;
    if res.is_some() {
        println!(
            "n1.created_at: {}",
            res.unwrap().get_datetime("created_at").unwrap()
        );
    }
    Ok(())
}

fn find_many(col: &Collection) -> Result<(), Error> {
    let find_options = FindOptions::builder().sort(doc! { "name": -1}).build();
    let a: Vec<Result<Document, Error>> = col.find(doc! {}, find_options)?.collect();
    println!("all: {:#?}", a);
    Ok(())
}
