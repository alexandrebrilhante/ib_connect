use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCreditDerivativeQuoteQuote {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::EnrichmentFieldsCreditDerivativeQuoteValuesInner>>,
}

impl EnrichmentFieldsCreditDerivativeQuoteQuote {
    pub fn new() -> EnrichmentFieldsCreditDerivativeQuoteQuote {
        EnrichmentFieldsCreditDerivativeQuoteQuote {
            confidence_type: None,
            values: None,
        }
    }
}
/// Confidence level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfidenceType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "MACHINE")]
    Machine,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ConfidenceType {
    fn default() -> ConfidenceType {
        Self::User
    }
}
