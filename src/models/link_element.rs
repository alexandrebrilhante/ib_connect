use crate::models;
use serde::{Deserialize, Serialize};

/// LinkElement : Link Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkElement {
    /// Should read \"LINK\" for this element type
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "href")]
    pub href: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LinkElement {
    /// Link Element
    pub fn new(content_type: ContentType, href: String) -> LinkElement {
        LinkElement {
            content_type,
            href,
            title: None,
        }
    }
}
/// Should read \"LINK\" for this element type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "LINK")]
    Link,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Link
    }
}
