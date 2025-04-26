use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquityQuote {
    /// The price of a security
    #[serde(rename = "ask_price", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Bid price
    #[serde(rename = "bid_price", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
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

impl StructuredTemplateFieldsEquityQuote {
    pub fn new() -> StructuredTemplateFieldsEquityQuote {
        StructuredTemplateFieldsEquityQuote {
            ask_price: None,
            bid_price: None,
            if_natural: None,
            price: None,
            order_type: None,
        }
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
