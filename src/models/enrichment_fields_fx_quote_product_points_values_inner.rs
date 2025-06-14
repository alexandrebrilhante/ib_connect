use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxQuoteProductPointsValuesInner : Rate given as points (scaled spread to the spot rate) with an associated type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxQuoteProductPointsValuesInner {
    /// Type of the quoted price
    #[serde(rename = "points_type", skip_serializing_if = "Option::is_none")]
    pub points_type: Option<PointsType>,
    /// Quoted points for the leg
    #[serde(rename = "points_value", skip_serializing_if = "Option::is_none")]
    pub points_value: Option<f64>,
}

impl EnrichmentFieldsFxQuoteProductPointsValuesInner {
    /// Rate given as points (scaled spread to the spot rate) with an associated type
    pub fn new() -> EnrichmentFieldsFxQuoteProductPointsValuesInner {
        EnrichmentFieldsFxQuoteProductPointsValuesInner {
            points_type: None,
            points_value: None,
        }
    }
}
/// Type of the quoted price
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PointsType {
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
}

impl Default for PointsType {
    fn default() -> PointsType {
        Self::Bid
    }
}
