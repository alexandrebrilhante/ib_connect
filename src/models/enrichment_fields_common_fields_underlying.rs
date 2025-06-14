use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonFieldsUnderlying {
    /// Reference price of underlying asset
    #[serde(rename = "reference_price", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<f64>,
    #[serde(rename = "reference_type", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ReferenceType>,
}

impl EnrichmentFieldsCommonFieldsUnderlying {
    pub fn new() -> EnrichmentFieldsCommonFieldsUnderlying {
        EnrichmentFieldsCommonFieldsUnderlying {
            reference_price: None,
            reference_type: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReferenceType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "FORWARD")]
    Forward,
}

impl Default for ReferenceType {
    fn default() -> ReferenceType {
        Self::Spot
    }
}
