use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxQuote {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Collection of points' values if several types are given (e.g. bid and ask)
    #[serde(
        rename = "product_points_values",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_points_values: Option<Vec<models::EnrichmentFieldsFxQuoteProductPointsValuesInner>>,
    /// Collection of al-in rates if several types are given (e.g. bid and ask)
    #[serde(
        rename = "product_all_in_rates",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_all_in_rates: Option<Vec<models::EnrichmentFieldsFxQuoteProductAllInRatesInner>>,
    /// Collection of rate data for instrument legs
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::EnrichmentFieldsFxQuoteLegsInner>>,
}

impl EnrichmentFieldsFxQuote {
    pub fn new() -> EnrichmentFieldsFxQuote {
        EnrichmentFieldsFxQuote {
            confidence_type: None,
            product_points_values: None,
            product_all_in_rates: None,
            legs: None,
        }
    }
}
/// Confidence level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfidenceType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "MACHINE")]
    Machine,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ConfidenceType {
    fn default() -> ConfidenceType {
        Self::User
    }
}
