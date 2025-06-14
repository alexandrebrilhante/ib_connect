use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOtherQuoteValuesInner {
    /// The price of a security
    #[serde(rename = "ask_value", skip_serializing_if = "Option::is_none")]
    pub ask_value: Option<f64>,
    /// The price of a security
    #[serde(rename = "bid_value", skip_serializing_if = "Option::is_none")]
    pub bid_value: Option<f64>,
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<ValueType>,
}

impl StructuredTemplateFieldsOtherQuoteValuesInner {
    pub fn new() -> StructuredTemplateFieldsOtherQuoteValuesInner {
        StructuredTemplateFieldsOtherQuoteValuesInner {
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
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::Price
    }
}
