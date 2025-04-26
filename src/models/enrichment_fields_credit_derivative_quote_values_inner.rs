use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCreditDerivativeQuoteValuesInner {
    /// Ask value of a price_type for the quote
    #[serde(rename = "ask_value", skip_serializing_if = "Option::is_none")]
    pub ask_value: Option<f64>,
    /// Bid value of a price_type for the quote
    #[serde(rename = "bid_value", skip_serializing_if = "Option::is_none")]
    pub bid_value: Option<f64>,
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<ValueType>,
}

impl EnrichmentFieldsCreditDerivativeQuoteValuesInner {
    pub fn new() -> EnrichmentFieldsCreditDerivativeQuoteValuesInner {
        EnrichmentFieldsCreditDerivativeQuoteValuesInner {
            ask_value: None,
            bid_value: None,
            value_type: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "UPFRONT")]
    Upfront,
    #[serde(rename = "SPREAD")]
    Spread,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::Price
    }
}
