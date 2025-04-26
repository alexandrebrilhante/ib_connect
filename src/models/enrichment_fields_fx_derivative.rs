use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxDerivative {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsFxDerivativeInstrumentInstrument>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsCreditDerivativeIntent>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::EnrichmentFieldsCreditDerivativeSide>>,
}

impl EnrichmentFieldsFxDerivative {
    pub fn new(at_type: AtType) -> EnrichmentFieldsFxDerivative {
        EnrichmentFieldsFxDerivative {
            at_type,
            instrument: None,
            intent: None,
            side: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "fx_derivative")]
    FxDerivative,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::FxDerivative
    }
}
