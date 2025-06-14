use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxInstrument {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Three letter Currency identifier
    #[serde(rename = "base_currency", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
    /// Three letter Currency identifier
    #[serde(rename = "variable_currency", skip_serializing_if = "Option::is_none")]
    pub variable_currency: Option<String>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi_underlying", skip_serializing_if = "Option::is_none")]
    pub figi_underlying: Option<String>,
    /// Type of an FX deal. NDF and NDS have 'delivery' set to 'NON-DELIVERABLE'
    #[serde(rename = "deal_type", skip_serializing_if = "Option::is_none")]
    pub deal_type: Option<DealType>,
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::EnrichmentFieldsFxInstrumentLegsInner>>,
}

impl EnrichmentFieldsFxInstrument {
    pub fn new() -> EnrichmentFieldsFxInstrument {
        EnrichmentFieldsFxInstrument {
            confidence_type: None,
            base_currency: None,
            variable_currency: None,
            figi_underlying: None,
            deal_type: None,
            legs: None,
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
/// Type of an FX deal. NDF and NDS have 'delivery' set to 'NON-DELIVERABLE'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DealType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "OUTRIGHT")]
    Outright,
    #[serde(rename = "SWAP")]
    Swap,
}

impl Default for DealType {
    fn default() -> DealType {
        Self::Spot
    }
}
