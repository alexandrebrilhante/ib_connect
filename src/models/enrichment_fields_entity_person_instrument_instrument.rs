use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsEntityPersonInstrumentInstrument {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// UUID
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<f64>,
}

impl EnrichmentFieldsEntityPersonInstrumentInstrument {
    pub fn new() -> EnrichmentFieldsEntityPersonInstrumentInstrument {
        EnrichmentFieldsEntityPersonInstrumentInstrument {
            confidence_type: None,
            uuid: None,
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
