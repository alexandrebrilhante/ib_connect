use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsCommonGroupsSize : Generic group Size
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonGroupsSize {
    /// Ask size (in security currency)
    #[serde(rename = "ask_size", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<f64>,
    /// Bid size (in security currency)
    #[serde(rename = "bid_size", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<f64>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// The size of a deal
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// The size of a deal in shares
    #[serde(rename = "share_size", skip_serializing_if = "Option::is_none")]
    pub share_size: Option<f64>,
    /// The size of a deal in notional ammount
    #[serde(rename = "notional_size", skip_serializing_if = "Option::is_none")]
    pub notional_size: Option<f64>,
}

impl EnrichmentFieldsCommonGroupsSize {
    /// Generic group Size
    pub fn new() -> EnrichmentFieldsCommonGroupsSize {
        EnrichmentFieldsCommonGroupsSize {
            ask_size: None,
            bid_size: None,
            confidence_type: None,
            size: None,
            share_size: None,
            notional_size: None,
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
