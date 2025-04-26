use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsOptionQuote {
    /// Reference price of underlying asset
    #[serde(rename = "reference_price", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<f64>,
    /// Bid price
    #[serde(rename = "bid_price", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// The price of a security
    #[serde(rename = "ask_price", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// The price of a security
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Represents a percentage and should always be normalized into a value between 0.00 and 1.00
    #[serde(rename = "normalized_delta", skip_serializing_if = "Option::is_none")]
    pub normalized_delta: Option<f64>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
}

impl EnrichmentFieldsOptionQuote {
    pub fn new() -> EnrichmentFieldsOptionQuote {
        EnrichmentFieldsOptionQuote {
            reference_price: None,
            bid_price: None,
            ask_price: None,
            price: None,
            normalized_delta: None,
            confidence_type: None,
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
