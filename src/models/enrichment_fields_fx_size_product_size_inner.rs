use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxSizeProductSizeInner : Amount with associated currency
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxSizeProductSizeInner {
    /// Quantity
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Currency of amount being traded
    #[serde(rename = "amount_currency", skip_serializing_if = "Option::is_none")]
    pub amount_currency: Option<String>,
    /// Side of an FX trade BID (also known as LHS) or ASK (also known as OFFER or RHS)
    #[serde(rename = "amount_side", skip_serializing_if = "Option::is_none")]
    pub amount_side: Option<AmountSide>,
}

impl EnrichmentFieldsFxSizeProductSizeInner {
    /// Amount with associated currency
    pub fn new() -> EnrichmentFieldsFxSizeProductSizeInner {
        EnrichmentFieldsFxSizeProductSizeInner {
            amount: None,
            amount_currency: None,
            amount_side: None,
        }
    }
}
/// Side of an FX trade BID (also known as LHS) or ASK (also known as OFFER or RHS)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AmountSide {
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
    #[serde(rename = "TWO-WAY")]
    TwoWay,
}

impl Default for AmountSide {
    fn default() -> AmountSide {
        Self::Bid
    }
}
