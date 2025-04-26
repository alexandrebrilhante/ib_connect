use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCommentaryBodyBlock {
    Paragraph(Box<models::Paragraph>),
}

impl Default for MarketCommentaryBodyBlock {
    fn default() -> Self {
        Self::Paragraph(Default::default())
    }
}
