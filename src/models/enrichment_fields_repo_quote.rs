use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsRepoQuote {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Rate
    #[serde(rename = "repo_rate", skip_serializing_if = "Option::is_none")]
    pub repo_rate: Option<f64>,
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    #[serde(rename = "termination_date", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
}

impl EnrichmentFieldsRepoQuote {
    pub fn new() -> EnrichmentFieldsRepoQuote {
        EnrichmentFieldsRepoQuote {
            confidence_type: None,
            repo_rate: None,
            settlement_date: None,
            termination_date: None,
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
