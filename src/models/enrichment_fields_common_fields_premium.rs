use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsCommonFieldsPremium : Option volatility
/// Option volatility
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnrichmentFieldsCommonFieldsPremium {
    EnrichmentFieldsCommonFieldsPremiumOneOf(Box<models::EnrichmentFieldsCommonFieldsPremiumOneOf>),
    EnrichmentFieldsCommonFieldsPremiumOneOf1(
        Box<models::EnrichmentFieldsCommonFieldsPremiumOneOf1>,
    ),
}

impl Default for EnrichmentFieldsCommonFieldsPremium {
    fn default() -> Self {
        Self::EnrichmentFieldsCommonFieldsPremiumOneOf(Default::default())
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
