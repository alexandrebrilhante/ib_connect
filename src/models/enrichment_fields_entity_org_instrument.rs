use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsEntityOrgInstrument {
    /// Bloomberg Company Identifier
    #[serde(rename = "bbid", skip_serializing_if = "Option::is_none")]
    pub bbid: Option<String>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Primary security FIGI identifier
    #[serde(rename = "primary_figi", skip_serializing_if = "Option::is_none")]
    pub primary_figi: Option<String>,
}

impl EnrichmentFieldsEntityOrgInstrument {
    pub fn new() -> EnrichmentFieldsEntityOrgInstrument {
        EnrichmentFieldsEntityOrgInstrument {
            bbid: None,
            confidence_type: None,
            primary_figi: None,
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
