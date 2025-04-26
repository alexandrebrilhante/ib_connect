use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonFieldsPremiumOneOf1 {
    #[serde(rename = "premium_type", skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    /// Three letter Currency identifier
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    /// The price of a security
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl EnrichmentFieldsCommonFieldsPremiumOneOf1 {
    pub fn new() -> EnrichmentFieldsCommonFieldsPremiumOneOf1 {
        EnrichmentFieldsCommonFieldsPremiumOneOf1 {
            premium_type: None,
            currency: None,
            at_type: None,
            value: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PremiumType {
    #[serde(rename = "PERCENT")]
    Percent,
    #[serde(rename = "PIPS")]
    Pips,
    #[serde(rename = "AMOUNT")]
    Amount,
}

impl Default for PremiumType {
    fn default() -> PremiumType {
        Self::Percent
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
