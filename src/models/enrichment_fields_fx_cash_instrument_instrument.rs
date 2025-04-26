use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxCashInstrumentInstrument {
    /// Bloomberg Company Identifier
    #[serde(rename = "bbid", skip_serializing_if = "Option::is_none")]
    pub bbid: Option<String>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Currency pair
    #[serde(rename = "currency_pair", skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    #[serde(rename = "deal_type", skip_serializing_if = "Option::is_none")]
    pub deal_type: Option<DealType>,
    #[serde(rename = "far_leg", skip_serializing_if = "Option::is_none")]
    pub far_leg: Option<Box<models::EnrichmentFieldsFxCashInstrumentLeg>>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi", skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    #[serde(rename = "leg", skip_serializing_if = "Option::is_none")]
    pub leg: Option<Box<models::EnrichmentFieldsFxCashInstrumentLeg>>,
    #[serde(rename = "near_leg", skip_serializing_if = "Option::is_none")]
    pub near_leg: Option<Box<models::EnrichmentFieldsFxCashInstrumentLeg>>,
}

impl EnrichmentFieldsFxCashInstrumentInstrument {
    pub fn new() -> EnrichmentFieldsFxCashInstrumentInstrument {
        EnrichmentFieldsFxCashInstrumentInstrument {
            bbid: None,
            confidence_type: None,
            currency_pair: None,
            deal_type: None,
            far_leg: None,
            figi: None,
            leg: None,
            near_leg: None,
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DealType {
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "NDF")]
    Ndf,
    #[serde(rename = "OUTRIGHT")]
    Outright,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
}

impl Default for DealType {
    fn default() -> DealType {
        Self::Depo
    }
}
