use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsMortgageQuoteValuesInner {
    /// Ask value of a price_type for the quote
    #[serde(rename = "ask_value", skip_serializing_if = "Option::is_none")]
    pub ask_value: Option<f64>,
    /// Bid value of a price_type for the quote
    #[serde(rename = "bid_value", skip_serializing_if = "Option::is_none")]
    pub bid_value: Option<f64>,
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<ValueType>,
}

impl EnrichmentFieldsMortgageQuoteValuesInner {
    pub fn new() -> EnrichmentFieldsMortgageQuoteValuesInner {
        EnrichmentFieldsMortgageQuoteValuesInner {
            ask_value: None,
            bid_value: None,
            value_type: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "PAYUP")]
    Payup,
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "YIELD")]
    Yield,
    #[serde(rename = "DISCOUNT_MARGIN")]
    DiscountMargin,
    #[serde(rename = "A_SPREAD")]
    ASpread,
    #[serde(rename = "E_SPREAD")]
    ESpread,
    #[serde(rename = "I_SPREAD")]
    ISpread,
    #[serde(rename = "J_SPREAD")]
    JSpread,
    #[serde(rename = "N_SPREAD")]
    NSpread,
    #[serde(rename = "S_SPREAD")]
    SSpread,
    #[serde(rename = "U_SPREAD")]
    USpread,
    #[serde(rename = "Z_SPREAD")]
    ZSpread,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::Payup
    }
}
