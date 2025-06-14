use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsSecurityLendingIntent : Generic group Intent
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsSecurityLendingIntent {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Name of the security-level intent
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}

impl EnrichmentFieldsSecurityLendingIntent {
    /// Generic group Intent
    pub fn new() -> EnrichmentFieldsSecurityLendingIntent {
        EnrichmentFieldsSecurityLendingIntent {
            confidence_type: None,
            name: None,
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
/// Name of the security-level intent
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "Inquiry")]
    Inquiry,
    #[serde(rename = "Quote")]
    Quote,
}

impl Default for Name {
    fn default() -> Name {
        Self::Inquiry
    }
}
