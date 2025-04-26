use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsSecurityLendingQuote {
    /// Rate
    #[serde(rename = "ask_rate", skip_serializing_if = "Option::is_none")]
    pub ask_rate: Option<f64>,
    /// Rate
    #[serde(rename = "bid_rate", skip_serializing_if = "Option::is_none")]
    pub bid_rate: Option<f64>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Rate
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "start_term", skip_serializing_if = "Option::is_none")]
    pub start_term: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTerm>>,
    #[serde(rename = "end_term", skip_serializing_if = "Option::is_none")]
    pub end_term: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTerm>>,
}

impl EnrichmentFieldsSecurityLendingQuote {
    pub fn new() -> EnrichmentFieldsSecurityLendingQuote {
        EnrichmentFieldsSecurityLendingQuote {
            ask_rate: None,
            bid_rate: None,
            confidence_type: None,
            rate: None,
            start_term: None,
            end_term: None,
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
