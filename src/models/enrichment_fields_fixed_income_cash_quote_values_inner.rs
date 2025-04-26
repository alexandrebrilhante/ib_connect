use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFixedIncomeCashQuoteValuesInner {
    /// Ask value of a price_type for the quote
    #[serde(rename = "ask_value", skip_serializing_if = "Option::is_none")]
    pub ask_value: Option<f64>,
    /// Bid value of a price_type for the quote
    #[serde(rename = "bid_value", skip_serializing_if = "Option::is_none")]
    pub bid_value: Option<f64>,
    /// The quote execution strategy
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<ValueType>,
}

impl EnrichmentFieldsFixedIncomeCashQuoteValuesInner {
    pub fn new() -> EnrichmentFieldsFixedIncomeCashQuoteValuesInner {
        EnrichmentFieldsFixedIncomeCashQuoteValuesInner {
            ask_value: None,
            bid_value: None,
            order_type: None,
            value_type: None,
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "YIELD")]
    Yield,
    #[serde(rename = "DISCOUNT")]
    Discount,
    #[serde(rename = "DISCOUNT_MARGIN")]
    DiscountMargin,
    #[serde(rename = "SPREAD_TO_BENCHMARK")]
    SpreadToBenchmark,
    #[serde(rename = "G_SPREAD")]
    GSpread,
    #[serde(rename = "I_SPREAD")]
    ISpread,
    #[serde(rename = "Z_SPREAD")]
    ZSpread,
    #[serde(rename = "ASW_SPREAD")]
    AswSpread,
    #[serde(rename = "OAS_SPREAD")]
    OasSpread,
    #[serde(rename = "CDS_BASIS")]
    CdsBasis,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::Price
    }
}
