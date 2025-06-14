use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxSizeLegsInner {
    /// Free-form string to identify leg information in various sections
    #[serde(rename = "leg_id", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<models::EnrichmentFieldsFxSizeProductSizeInner>>,
}

impl EnrichmentFieldsFxSizeLegsInner {
    pub fn new() -> EnrichmentFieldsFxSizeLegsInner {
        EnrichmentFieldsFxSizeLegsInner {
            leg_id: None,
            size: None,
        }
    }
}
