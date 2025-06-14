use crate::models;
use serde::{Deserialize, Serialize};

/// MarketCommentaryElement : A structured element for commentary or analysis of a market event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryElement {
    /// (This is currently an experimental feature and needs firm specific enablement)
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    /// The title of the Market Commentary.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "body")]
    pub body: Box<models::MarketCommentaryBodyRichText>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::MarketCommentaryMetadata>>,
}

impl MarketCommentaryElement {
    /// A structured element for commentary or analysis of a market event.
    pub fn new(
        content_type: ContentType,
        title: String,
        body: models::MarketCommentaryBodyRichText,
    ) -> MarketCommentaryElement {
        MarketCommentaryElement {
            content_type,
            title,
            body: Box::new(body),
            metadata: None,
        }
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
