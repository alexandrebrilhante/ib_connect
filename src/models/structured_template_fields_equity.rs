use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquity {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument")]
    pub instrument: Box<models::StructuredTemplateFieldsEquityInstrument>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::StructuredTemplateFieldsEquityQuote>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::StructuredTemplateFieldsEquityIntent>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::StructuredTemplateFieldsEquitySide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::StructuredTemplateFieldsEquitySize>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::StructuredTemplateFieldsEquityNote>>,
}

impl StructuredTemplateFieldsEquity {
    pub fn new(
        at_type: AtType,
        instrument: models::StructuredTemplateFieldsEquityInstrument,
    ) -> StructuredTemplateFieldsEquity {
        StructuredTemplateFieldsEquity {
            at_type,
            instrument: Box::new(instrument),
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
    #[serde(rename = "equity")]
    Equity,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Equity
    }
}
