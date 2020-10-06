use mongodb::bson::{Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DataStatus {
    New,
    Ok,
    Error,
}

impl ToString for DataStatus {
    fn to_string(&self) -> String {
        match self {
            DataStatus::New => String::from("NEW"),
            DataStatus::Ok => String::from("OK"),
            DataStatus::Error => String::from("ERROR"),
        }
    }
}

impl Into<Bson> for DataStatus {
    fn into(self) -> Bson {
        Bson::String(self.to_string())
    }
}

fn main() {
    let mut filter = Document::new();
    filter.insert("status", DataStatus::Ok);
    println!("{}", filter);
}
