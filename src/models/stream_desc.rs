use crate::models;
use serde::{Deserialize, Serialize};

/// StreamDesc : An object describing a single stream.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamDesc {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "firmId", skip_serializing_if = "Option::is_none")]
    pub firm_id: Option<i32>,
}

impl StreamDesc {
    /// An object describing a single stream.
    pub fn new(id: String) -> StreamDesc {
        StreamDesc {
            id,
            display_name: None,
            description: None,
            firm_id: None,
        }
    }
}
