use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsEquitySide : Generic group Side. Most of the types use this one
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquitySide {
    /// Side of the quote or security mention
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

impl StructuredTemplateFieldsEquitySide {
    /// Generic group Side. Most of the types use this one
    pub fn new() -> StructuredTemplateFieldsEquitySide {
        StructuredTemplateFieldsEquitySide { side: None }
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
    #[serde(rename = "SELL-SHORT")]
    SellShort,
    #[serde(rename = "BUY-TO-COVER")]
    BuyToCover,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}
