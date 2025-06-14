use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxQuoteLegsInner : Rate information for a given instrument leg
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxQuoteLegsInner {
    /// Free-form string to identify leg information in various sections
    #[serde(rename = "leg_id", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<String>,
    /// Collection of points' values if several types are given (e.g. bid and ask)
    #[serde(rename = "leg_points_values", skip_serializing_if = "Option::is_none")]
    pub leg_points_values: Option<Vec<models::EnrichmentFieldsFxQuoteProductPointsValuesInner>>,
    /// Collection of al-in rates if several types are given (e.g. bid and ask)
    #[serde(rename = "leg_all_in_rates", skip_serializing_if = "Option::is_none")]
    pub leg_all_in_rates: Option<Vec<models::EnrichmentFieldsFxQuoteProductAllInRatesInner>>,
}

impl EnrichmentFieldsFxQuoteLegsInner {
    /// Rate information for a given instrument leg
    pub fn new() -> EnrichmentFieldsFxQuoteLegsInner {
        EnrichmentFieldsFxQuoteLegsInner {
            leg_id: None,
            leg_points_values: None,
            leg_all_in_rates: None,
        }
    }
}
