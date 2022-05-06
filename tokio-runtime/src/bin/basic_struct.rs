use mongodb::Client;

struct Data {
    
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Document>("data");
    collection.drop(None).await?;
    Ok(())
}