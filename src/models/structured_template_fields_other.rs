use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOther {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument")]
    pub instrument: Box<models::StructuredTemplateFieldsOtherInstrument>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::StructuredTemplateFieldsOtherSide>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::StructuredTemplateFieldsOtherQuote>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::StructuredTemplateFieldsEquityIntent>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSize>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::StructuredTemplateFieldsEquityNote>>,
}

impl StructuredTemplateFieldsOther {
    pub fn new(
        at_type: AtType,
        instrument: models::StructuredTemplateFieldsOtherInstrument,
    ) -> StructuredTemplateFieldsOther {
        StructuredTemplateFieldsOther {
            at_type,
            instrument: Box::new(instrument),
            side: None,
            quote: None,
            intent: None,
            size: None,
            note: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "other")]
    Other,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Other
    }
}
