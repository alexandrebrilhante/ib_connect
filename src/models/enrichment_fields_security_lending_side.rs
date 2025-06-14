use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsSecurityLendingSide : Generic group Side. Most of the types use this one
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsSecurityLendingSide {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Side of the quote or security mention
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

impl EnrichmentFieldsSecurityLendingSide {
    /// Generic group Side. Most of the types use this one
    pub fn new() -> EnrichmentFieldsSecurityLendingSide {
        EnrichmentFieldsSecurityLendingSide {
            confidence_type: None,
            side: None,
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
/// Side of the quote or security mention
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORROW")]
    Borrow,
}

impl Default for Side {
    fn default() -> Side {
        Self::Lend
    }
}
