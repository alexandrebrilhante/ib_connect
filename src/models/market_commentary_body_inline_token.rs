use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCommentaryBodyInlineToken {
    TextToken(Box<models::Text>),
    LinkToken(Box<models::Link>),
}

impl Default for MarketCommentaryBodyInlineToken {
    fn default() -> Self {
        Self::TextToken(Default::default())
    }
}
