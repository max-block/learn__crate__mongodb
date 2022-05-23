use std::io::Error;

use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let database_url = "mongodb://localhost/learn__crate__mongodb";
    let client = Client::with_uri_str(database_url);
    let database_name = ClientOptions::parse(database_url).await?.default_database.unwrap();
    dbg!(database_name);
    let database = client.database("learn__create__mongodb");

    let collection = database.collection::<Document>("data");
    collection.drop(None).await?;
    Ok(())
}
