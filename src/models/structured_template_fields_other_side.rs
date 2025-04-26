use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOtherSide {
    /// Side of the quote or security mention
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

impl StructuredTemplateFieldsOtherSide {
    pub fn new() -> StructuredTemplateFieldsOtherSide {
        StructuredTemplateFieldsOtherSide { side: None }
    }
}
/// Side of the quote or security mention
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWO-SIDED")]
    TwoSided,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}
