use crate::models;
use serde::{Deserialize, Serialize};

/// MessageElement : An element of a message that can be combined to form rich content messages.
/// An element of a message that can be combined to form rich content messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageElement {
    TextElement(Box<models::TextElement>),
    LinkElement(Box<models::LinkElement>),
    MentionElement(Box<models::MentionElement>),
    TableElement(Box<models::TableElement>),
    CardElement(Box<models::CardElement>),
}

impl Default for MessageElement {
    fn default() -> Self {
        Self::TextElement(Default::default())
    }
}
/// (This is currently an experimental feature and needs firm specific enablement)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "ORDER UPDATE")]
    OrderUpdate,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::OrderUpdate
    }
}
