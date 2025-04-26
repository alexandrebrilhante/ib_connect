use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsCreditDerivativeIntent : Generic group Intent
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCreditDerivativeIntent {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Name of the security-level intent
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}

impl EnrichmentFieldsCreditDerivativeIntent {
    /// Generic group Intent
    pub fn new() -> EnrichmentFieldsCreditDerivativeIntent {
        EnrichmentFieldsCreditDerivativeIntent {
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
    #[serde(rename = "Order")]
    Order,
    #[serde(rename = "Quote")]
    Quote,
}

impl Default for Name {
    fn default() -> Name {
        Self::Inquiry
    }
}
