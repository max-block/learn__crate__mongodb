use mongodb::bson::{Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DataStatus {
    NEW,
    OK,
    ERROR,
}

// impl ToString for DataStatus {
//     fn to_string(&self) -> String {
//         match self {
//             DataStatus::NEW => String::from("NEW"),
//             DataStatus::OK => String::from("OK"),
//             DataStatus::ERROR => String::from("ERROR"),
//         }
//     }
// }

// impl Into<Bson> for DataStatus {
//     fn into(self) -> Bson {
//         Bson::String(self.to_string())
//     }
// }

fn main() {
    let mut filter = Document::new();
    filter.insert("status", DataStatus::OK);
    println!("{}", filter);
}
