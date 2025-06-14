use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxQuoteProductAllInRatesInner : All-in rate with an associated rate type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxQuoteProductAllInRatesInner {
    /// Type of the quoted price
    #[serde(rename = "all_in_rate_type", skip_serializing_if = "Option::is_none")]
    pub all_in_rate_type: Option<AllInRateType>,
    /// Rate
    #[serde(rename = "all_in_rate_value", skip_serializing_if = "Option::is_none")]
    pub all_in_rate_value: Option<f64>,
}

impl EnrichmentFieldsFxQuoteProductAllInRatesInner {
    /// All-in rate with an associated rate type
    pub fn new() -> EnrichmentFieldsFxQuoteProductAllInRatesInner {
        EnrichmentFieldsFxQuoteProductAllInRatesInner {
            all_in_rate_type: None,
            all_in_rate_value: None,
        }
    }
}
/// Type of the quoted price
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllInRateType {
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
}

impl Default for AllInRateType {
    fn default() -> AllInRateType {
        Self::Bid
    }
}
