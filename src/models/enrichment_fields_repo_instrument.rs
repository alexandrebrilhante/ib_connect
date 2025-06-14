use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsRepoInstrument {
    /// Bloomberg Company Identifier
    #[serde(rename = "bbid", skip_serializing_if = "Option::is_none")]
    pub bbid: Option<String>,
    /// Bloomberg Company Identifier
    #[serde(rename = "collateral", skip_serializing_if = "Option::is_none")]
    pub collateral: Option<String>,
    /// Quantity
    #[serde(rename = "face_amount", skip_serializing_if = "Option::is_none")]
    pub face_amount: Option<f64>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi", skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi_underlying", skip_serializing_if = "Option::is_none")]
    pub figi_underlying: Option<String>,
    /// True if the repo does not have a fixed maturity date
    #[serde(rename = "is_open", skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    /// True if the repo trade is carried over from a prior trade
    #[serde(rename = "is_roll", skip_serializing_if = "Option::is_none")]
    pub is_roll: Option<bool>,
}

impl EnrichmentFieldsRepoInstrument {
    pub fn new() -> EnrichmentFieldsRepoInstrument {
        EnrichmentFieldsRepoInstrument {
            bbid: None,
            collateral: None,
            face_amount: None,
            figi: None,
            figi_underlying: None,
            is_open: None,
            is_roll: None,
        }
    }
}
