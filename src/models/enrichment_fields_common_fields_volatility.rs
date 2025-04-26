use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsCommonFieldsVolatility : Option volatility
/// Option volatility
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnrichmentFieldsCommonFieldsVolatility {
    EnrichmentFieldsCommonFieldsVolatilityOneOf(
        Box<models::EnrichmentFieldsCommonFieldsVolatilityOneOf>,
    ),
    EnrichmentFieldsCommonFieldsVolatilityOneOf1(
        Box<models::EnrichmentFieldsCommonFieldsVolatilityOneOf1>,
    ),
}

impl Default for EnrichmentFieldsCommonFieldsVolatility {
    fn default() -> Self {
        Self::EnrichmentFieldsCommonFieldsVolatilityOneOf(Default::default())
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
