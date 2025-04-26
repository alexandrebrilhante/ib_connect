use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryMetadata {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<models::ListedSecurity>>,
}

impl MarketCommentaryMetadata {
    pub fn new() -> MarketCommentaryMetadata {
        MarketCommentaryMetadata { instruments: None }
    }
}
