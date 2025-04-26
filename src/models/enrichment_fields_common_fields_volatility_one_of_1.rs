use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonFieldsVolatilityOneOf1 {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    /// The price of a security
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl EnrichmentFieldsCommonFieldsVolatilityOneOf1 {
    pub fn new() -> EnrichmentFieldsCommonFieldsVolatilityOneOf1 {
        EnrichmentFieldsCommonFieldsVolatilityOneOf1 {
            at_type: None,
            value: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "value")]
    Value,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Value
    }
}
