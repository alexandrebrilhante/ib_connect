use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsEquityQuoteQuote {
    /// The price of a security
    #[serde(rename = "ask_price", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Bid price
    #[serde(rename = "bid_price", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Natural Indication of Interest (see FINRA regulatory notice 11-43).
    #[serde(rename = "if_natural", skip_serializing_if = "Option::is_none")]
    pub if_natural: Option<bool>,
    /// The price of a security
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// The quote execution strategy
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
}

impl EnrichmentFieldsEquityQuoteQuote {
    pub fn new() -> EnrichmentFieldsEquityQuoteQuote {
        EnrichmentFieldsEquityQuoteQuote {
            ask_price: None,
            bid_price: None,
            confidence_type: None,
            if_natural: None,
            price: None,
            order_type: None,
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
/// The quote execution strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}

impl Default for OrderType {
    fn default() -> OrderType {
        Self::Limit
    }
}
