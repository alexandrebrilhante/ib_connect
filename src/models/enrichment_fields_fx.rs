use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFx {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsFxInstrument>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::EnrichmentFieldsFxQuote>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::EnrichmentFieldsFxSize>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::EnrichmentFieldsFxSide>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsFxIntent>>,
}

impl EnrichmentFieldsFx {
    pub fn new(at_type: AtType) -> EnrichmentFieldsFx {
        EnrichmentFieldsFx {
            at_type,
            instrument: None,
            quote: None,
            size: None,
            side: None,
            intent: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "fx")]
    Fx,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Fx
    }
}
