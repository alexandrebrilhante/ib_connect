use crate::models;
use serde::{Deserialize, Serialize};

/// IdeaTextElement : An element of an idea that can be combined to form rich text messages.
/// An element of an idea that can be combined to form rich text messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdeaTextElement {
    TextElement(Box<models::TextElement>),
    LinkElement(Box<models::LinkElement>),
}

impl Default for IdeaTextElement {
    fn default() -> Self {
        Self::TextElement(Default::default())
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
