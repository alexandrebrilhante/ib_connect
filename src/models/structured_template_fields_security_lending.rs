use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsSecurityLending {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument")]
    pub instrument: Box<models::StructuredTemplateFieldsSecurityLendingInstrument>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuote>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::StructuredTemplateFieldsSecurityLendingSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSize>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::StructuredTemplateFieldsEquityIntent>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::StructuredTemplateFieldsEquityNote>>,
}

impl StructuredTemplateFieldsSecurityLending {
    pub fn new(
        at_type: AtType,
        instrument: models::StructuredTemplateFieldsSecurityLendingInstrument,
    ) -> StructuredTemplateFieldsSecurityLending {
        StructuredTemplateFieldsSecurityLending {
            at_type,
            instrument: Box::new(instrument),
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
