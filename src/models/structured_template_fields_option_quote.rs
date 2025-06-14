use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOptionQuote {
    /// Reference price of underlying asset
    #[serde(rename = "reference_price", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<f64>,
    /// Bid price
    #[serde(rename = "bid_price", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// The price of a security
    #[serde(rename = "ask_price", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Represents a percentage and should always be normalized into a value between 0.00 and 1.00
    #[serde(rename = "normalized_delta", skip_serializing_if = "Option::is_none")]
    pub normalized_delta: Option<f64>,
}

impl StructuredTemplateFieldsOptionQuote {
    pub fn new() -> StructuredTemplateFieldsOptionQuote {
        StructuredTemplateFieldsOptionQuote {
            reference_price: None,
            bid_price: None,
            ask_price: None,
            normalized_delta: None,
        }
    }
}
