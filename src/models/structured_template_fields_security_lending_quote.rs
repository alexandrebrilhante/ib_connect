use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsSecurityLendingQuote {
    /// Rate
    #[serde(rename = "ask_rate", skip_serializing_if = "Option::is_none")]
    pub ask_rate: Option<f64>,
    /// Rate
    #[serde(rename = "bid_rate", skip_serializing_if = "Option::is_none")]
    pub bid_rate: Option<f64>,
    /// Rate
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "start_term", skip_serializing_if = "Option::is_none")]
    pub start_term: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTerm>>,
    #[serde(rename = "end_term", skip_serializing_if = "Option::is_none")]
    pub end_term: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTerm>>,
}

impl StructuredTemplateFieldsSecurityLendingQuote {
    pub fn new() -> StructuredTemplateFieldsSecurityLendingQuote {
        StructuredTemplateFieldsSecurityLendingQuote {
            ask_rate: None,
            bid_rate: None,
            rate: None,
            start_term: None,
            end_term: None,
        }
    }
}
