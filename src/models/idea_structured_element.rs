use crate::models;
use serde::{Deserialize, Serialize};

/// IdeaStructuredElement : A rich content element that can be added to a suggestion.
/// A rich content element that can be added to a suggestion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdeaStructuredElement {
    TableElement(Box<models::TableElement>),
    MarketCommentaryElement(Box<models::MarketCommentaryElement>),
}

impl Default for IdeaStructuredElement {
    fn default() -> Self {
        Self::TableElement(Default::default())
    }
}
/// (This is currently an experimental feature and needs firm specific enablement)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "MARKET COMMENTARY")]
    MarketCommentary,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::MarketCommentary
    }
}
