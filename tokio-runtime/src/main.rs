use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Client,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Document>("data");
    collection.drop(None).await?;

    collection.insert_one(doc! {"name": "n1", "value": 1}, None).await?;
    collection
        .insert_many(vec![doc! {"name": "n2", "value": 2}], None)
        .await?;

    let all: Vec<mongodb::error::Result<Document>> = collection.find(doc! {}, None).await?.collect().await;
    dbg!(all);

    Ok(())
}
