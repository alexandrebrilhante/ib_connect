use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxCash {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsFxCashInstrumentInstrument>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::EnrichmentFieldsFxCashQuoteQuote>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::EnrichmentFieldsFxCashSizeSize>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsCreditDerivativeIntent>>,
}

impl EnrichmentFieldsFxCash {
    pub fn new(at_type: AtType) -> EnrichmentFieldsFxCash {
        EnrichmentFieldsFxCash {
            at_type,
            instrument: None,
            quote: None,
            size: None,
            intent: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "fx_cash")]
    FxCash,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::FxCash
    }
}
