use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonFieldsVolatilityOneOf {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    /// The price of a security
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<f64>,
    /// The price of a security
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<f64>,
}

impl EnrichmentFieldsCommonFieldsVolatilityOneOf {
    pub fn new() -> EnrichmentFieldsCommonFieldsVolatilityOneOf {
        EnrichmentFieldsCommonFieldsVolatilityOneOf {
            at_type: None,
            bid: None,
            ask: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "quote")]
    Quote,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Quote
    }
}
