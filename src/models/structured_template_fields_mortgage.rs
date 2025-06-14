use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsMortgage {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<models::StructuredTemplateFieldsMortgageHeader>>,
    #[serde(rename = "instrument")]
    pub instrument: Box<models::StructuredTemplateFieldsMortgageInstrument>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::StructuredTemplateFieldsMortgageQuote>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::StructuredTemplateFieldsEquityIntent>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::StructuredTemplateFieldsFixedIncomeCashSize>>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Box<models::StructuredTemplateFieldsEquityNote>>,
}

impl StructuredTemplateFieldsMortgage {
    pub fn new(
        at_type: AtType,
        instrument: models::StructuredTemplateFieldsMortgageInstrument,
    ) -> StructuredTemplateFieldsMortgage {
        StructuredTemplateFieldsMortgage {
            at_type,
            header: None,
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
    #[serde(rename = "mortgage")]
    Mortgage,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Mortgage
    }
}
