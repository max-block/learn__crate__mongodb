use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    sync::Client,
    IndexModel,
};

fn main() -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("learn__create__mongodb");
    let collection = database.collection::<Document>("data");
    collection.drop(None)?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        None,
    )?;

    let res = collection.insert_one(doc! {"name": "n1", "value": 1}, None);
    dbg!(&res);

    let res = collection.insert_one(doc! {"name": "n1", "value": 2}, None);
    dbg!(&res);
    // E11000 duplicate key error collection: learn__create__mongodb.data index: name_1 dup key: { name: \"n1\" }

    Ok(())
}
