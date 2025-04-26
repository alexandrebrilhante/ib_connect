use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCommentaryBodyLink {
    BbLink(Box<models::BloombergLink>),
    WebLink(Box<models::WebLink>),
}

impl Default for MarketCommentaryBodyLink {
    fn default() -> Self {
        Self::BbLink(Default::default())
    }
}
