use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsSecurityLending {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsSecurityLendingInstrument>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::EnrichmentFieldsSecurityLendingQuote>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::EnrichmentFieldsSecurityLendingSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::EnrichmentFieldsCreditDerivativeSize>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsSecurityLendingIntent>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::EnrichmentFieldsCommonGroupsNote>>,
}

impl EnrichmentFieldsSecurityLending {
    pub fn new(at_type: AtType) -> EnrichmentFieldsSecurityLending {
        EnrichmentFieldsSecurityLending {
            at_type,
            instrument: None,
            quote: None,
            side: None,
            size: None,
            intent: None,
            note: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "security_lending")]
    SecurityLending,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::SecurityLending
    }
}
