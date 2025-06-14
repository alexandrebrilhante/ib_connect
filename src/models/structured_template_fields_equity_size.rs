use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsEquitySize : Generic group Size
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquitySize {
    /// Ask size (in security currency)
    #[serde(rename = "ask_size", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<f64>,
    /// Bid size (in security currency)
    #[serde(rename = "bid_size", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<f64>,
    /// The size of a deal
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// The size of a deal in shares
    #[serde(rename = "share_size", skip_serializing_if = "Option::is_none")]
    pub share_size: Option<f64>,
    /// The size of a deal in notional ammount
    #[serde(rename = "notional_size", skip_serializing_if = "Option::is_none")]
    pub notional_size: Option<f64>,
}

impl StructuredTemplateFieldsEquitySize {
    /// Generic group Size
    pub fn new() -> StructuredTemplateFieldsEquitySize {
        StructuredTemplateFieldsEquitySize {
            ask_size: None,
            bid_size: None,
            size: None,
            share_size: None,
            notional_size: None,
        }
    }
}
