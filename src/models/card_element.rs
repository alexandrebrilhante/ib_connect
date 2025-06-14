use crate::models;
use serde::{Deserialize, Serialize};

/// CardElement : A rich content element that can be posted by a chatbot. Each chatbot message can contain only 1.
/// A rich content element that can be posted by a chatbot. Each chatbot message can contain only 1.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardElement {
    MarketCommentaryElement(Box<models::MarketCommentaryElement>),
    OrderUpdateElement(Box<models::OrderUpdateElement>),
}

impl Default for CardElement {
    fn default() -> Self {
        Self::MarketCommentaryElement(Default::default())
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
