use crate::models;
use serde::{Deserialize, Serialize};

/// WebLink : Represents an http or https link.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebLink {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The url has to be a valid link in URI format
    #[serde(rename = "url")]
    pub url: String,
}

impl WebLink {
    /// Represents an http or https link.
    pub fn new(r#type: Type, url: String) -> WebLink {
        WebLink { r#type, url }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WebLink")]
    WebLink,
}

impl Default for Type {
    fn default() -> Type {
        Self::WebLink
    }
}
