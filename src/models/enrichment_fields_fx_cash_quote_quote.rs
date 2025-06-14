use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxCashQuoteQuote {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Rate
    #[serde(rename = "spot_rate", skip_serializing_if = "Option::is_none")]
    pub spot_rate: Option<f64>,
}

impl EnrichmentFieldsFxCashQuoteQuote {
    pub fn new() -> EnrichmentFieldsFxCashQuoteQuote {
        EnrichmentFieldsFxCashQuoteQuote {
            confidence_type: None,
            spot_rate: None,
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
