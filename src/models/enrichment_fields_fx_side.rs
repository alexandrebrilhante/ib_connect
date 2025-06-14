use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxSide {
    /// Side of an FX trade BID (also known as LHS) or ASK (also known as OFFER or RHS)
    #[serde(rename = "product_side", skip_serializing_if = "Option::is_none")]
    pub product_side: Option<ProductSide>,
    /// Side information for the legs
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::EnrichmentFieldsFxSideLegsInner>>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
}

impl EnrichmentFieldsFxSide {
    pub fn new() -> EnrichmentFieldsFxSide {
        EnrichmentFieldsFxSide {
            product_side: None,
            legs: None,
            confidence_type: None,
        }
    }
}
/// Side of an FX trade BID (also known as LHS) or ASK (also known as OFFER or RHS)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductSide {
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
    #[serde(rename = "TWO-WAY")]
    TwoWay,
}

impl Default for ProductSide {
    fn default() -> ProductSide {
        Self::Bid
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
