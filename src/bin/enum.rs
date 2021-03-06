use mongodb::bson::{self, doc, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ERROR")]
    Error,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::New => String::from("NEW"),
            Status::Ok => String::from("OK"),
            Status::Error => String::from("ERROR"),
        }
    }
}

impl From<Status> for Bson {
    fn from(s: Status) -> Self {
        Bson::String(s.to_string())
    }
}

fn main() {
    let d = Data {
        name: "n1".to_string(),
        status: Status::Ok,
    };
    let doc = bson::to_document(&d).unwrap();
    dbg!(doc);

    let mut filter = Document::new();
    filter.insert("status", Status::Ok);
    dbg!(filter);

    let filter = doc! {"status": Status::Ok};
    dbg!(filter);
}
