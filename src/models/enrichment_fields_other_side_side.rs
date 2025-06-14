use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsOtherSideSide {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Side of the quote or security mention
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

impl EnrichmentFieldsOtherSideSide {
    pub fn new() -> EnrichmentFieldsOtherSideSide {
        EnrichmentFieldsOtherSideSide {
            confidence_type: None,
            side: None,
        }
    }
}
/// Confidence level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfidenceType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "MACHINE")]
    Machine,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ConfidenceType {
    fn default() -> ConfidenceType {
        Self::User
    }
}
/// Side of the quote or security mention
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORROW")]
    Borrow,
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
        Self::Lend
    }
}
