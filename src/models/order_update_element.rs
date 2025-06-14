use crate::models;
use serde::{Deserialize, Serialize};

/// OrderUpdateElement : A structured element for an Order Update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateElement {
    /// (This is currently an experimental feature and needs firm specific enablement)
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "body")]
    pub body: Box<models::OrderUpdateElementBody>,
}

impl OrderUpdateElement {
    /// A structured element for an Order Update.
    pub fn new(
        content_type: ContentType,
        body: models::OrderUpdateElementBody,
    ) -> OrderUpdateElement {
        OrderUpdateElement {
            content_type,
            body: Box::new(body),
        }
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
