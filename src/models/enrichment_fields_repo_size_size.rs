use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsRepoSizeSize {
    /// Ask size (in security currency)
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<f64>,
    /// Bid size (in security currency)
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<f64>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
}

impl EnrichmentFieldsRepoSizeSize {
    pub fn new() -> EnrichmentFieldsRepoSizeSize {
        EnrichmentFieldsRepoSizeSize {
            ask: None,
            bid: None,
            confidence_type: None,
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
