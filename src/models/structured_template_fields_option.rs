use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOption {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument")]
    pub instrument: Box<models::StructuredTemplateFieldsOptionInstrument>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::StructuredTemplateFieldsEquityIntent>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::StructuredTemplateFieldsOptionQuote>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSize>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::StructuredTemplateFieldsEquityNote>>,
}

impl StructuredTemplateFieldsOption {
    pub fn new(
        at_type: AtType,
        instrument: models::StructuredTemplateFieldsOptionInstrument,
    ) -> StructuredTemplateFieldsOption {
        StructuredTemplateFieldsOption {
            at_type,
            instrument: Box::new(instrument),
            intent: None,
            quote: None,
            side: None,
            size: None,
            note: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "option")]
    Option,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Option
    }
}
