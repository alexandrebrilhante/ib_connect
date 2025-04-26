use crate::models;
use serde::{Deserialize, Serialize};

/// JsonTable : An object that contains a JSON representation of a table.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonTable {
    #[serde(rename = "json")]
    pub json: Box<models::JsonTableJson>,
}

impl JsonTable {
    /// An object that contains a JSON representation of a table.
    pub fn new(json: models::JsonTableJson) -> JsonTable {
        JsonTable {
            json: Box::new(json),
        }
    }
}
