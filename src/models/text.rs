use crate::models;
use serde::{Deserialize, Serialize};

/// Text : Represents formatted text.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<models::TextFormattingOptions>>,
}

impl Text {
    /// Represents formatted text.
    pub fn new(r#type: Type, text: String) -> Text {
        Text {
            r#type,
            text,
            attributes: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Text")]
    Text,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}
