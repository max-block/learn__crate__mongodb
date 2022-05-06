use std::io::Error;

use mongodb::{Client, options::ClientOptions};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let database_url = "mongodb://localhost/learn__crate__mongodb";
    let client = Client::with_uri_str(database_url);
    let database_name = ClientOptions::parse(database_url).await?.default_database.unwrap();
    dbg!(database_name);
    Ok(())
}
