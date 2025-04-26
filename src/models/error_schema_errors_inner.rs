use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorSchemaErrorsInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ErrorSchemaErrorsInner {
    pub fn new(detail: String) -> ErrorSchemaErrorsInner {
        ErrorSchemaErrorsInner {
            id: None,
            detail,
            status: None,
            title: None,
        }
    }
}
