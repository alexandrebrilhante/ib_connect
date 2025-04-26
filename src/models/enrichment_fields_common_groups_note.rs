use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsCommonGroupsNote : Generic group Note
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonGroupsNote {
    /// Free-form text for specifying additional details in SPW negotiations
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
}

impl EnrichmentFieldsCommonGroupsNote {
    /// Generic group Note
    pub fn new() -> EnrichmentFieldsCommonGroupsNote {
        EnrichmentFieldsCommonGroupsNote {
            text: None,
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
