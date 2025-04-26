use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonTableJson {
    #[serde(rename = "rows")]
    pub rows: Vec<models::TableRow>,
}

impl JsonTableJson {
    pub fn new(rows: Vec<models::TableRow>) -> JsonTableJson {
        JsonTableJson { rows }
    }
}
