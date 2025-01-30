use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    r#type: String,
    #[serde(rename = "typeLabel")]
    type_label: String,
}
