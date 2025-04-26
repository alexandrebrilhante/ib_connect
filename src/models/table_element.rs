use crate::models;
use serde::{Deserialize, Serialize};

/// TableElement : Table Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableElement {
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "rows")]
    pub rows: Vec<models::TableRow>,
}

impl TableElement {
    /// Table Element
    pub fn new(content_type: ContentType, rows: Vec<models::TableRow>) -> TableElement {
        TableElement { content_type, rows }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "TABLE")]
    Table,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Table
    }
}
