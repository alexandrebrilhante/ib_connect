use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxInstrumentLegsInnerSettlementDate : Settlement date for the leg. Commonly referred to as Value Date
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxInstrumentLegsInnerSettlementDate {
    /// Tenor associated with security.
    #[serde(rename = "tenor", skip_serializing_if = "Option::is_none")]
    pub tenor: Option<String>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTermDate>>,
}

impl EnrichmentFieldsFxInstrumentLegsInnerSettlementDate {
    /// Settlement date for the leg. Commonly referred to as Value Date
    pub fn new() -> EnrichmentFieldsFxInstrumentLegsInnerSettlementDate {
        EnrichmentFieldsFxInstrumentLegsInnerSettlementDate {
            tenor: None,
            date: None,
        }
    }
}
