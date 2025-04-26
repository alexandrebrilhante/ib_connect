use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryBodyRichText {
    #[serde(rename = "blocks")]
    pub blocks: Vec<models::Block>,
}

impl MarketCommentaryBodyRichText {
    pub fn new(blocks: Vec<models::Block>) -> MarketCommentaryBodyRichText {
        MarketCommentaryBodyRichText { blocks }
    }
}
