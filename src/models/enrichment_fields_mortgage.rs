use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsMortgage {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsMortgageInstrument>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::EnrichmentFieldsMortgageQuote>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsCreditDerivativeIntent>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::EnrichmentFieldsCreditDerivativeSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::EnrichmentFieldsCreditDerivativeSize>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::EnrichmentFieldsCommonGroupsNote>>,
}

impl EnrichmentFieldsMortgage {
    pub fn new(at_type: AtType) -> EnrichmentFieldsMortgage {
        EnrichmentFieldsMortgage {
            at_type,
            instrument: None,
            quote: None,
            intent: None,
            side: None,
            size: None,
            note: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "mortgage")]
    Mortgage,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Mortgage
    }
}
