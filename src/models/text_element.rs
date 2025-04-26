use crate::models;
use serde::{Deserialize, Serialize};

/// TextElement : Text Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextElement {
    /// Should read \"TEXT\" for this element type
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "text")]
    pub text: String,
}

impl TextElement {
    /// Text Element
    pub fn new(content_type: ContentType, text: String) -> TextElement {
        TextElement { content_type, text }
    }
}
/// Should read \"TEXT\" for this element type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "TEXT")]
    Text,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Text
    }
}
