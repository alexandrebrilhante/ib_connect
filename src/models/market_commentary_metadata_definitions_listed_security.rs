use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryMetadataDefinitionsListedSecurity {
    /// Primary security FIGI identifier
    #[serde(rename = "figi")]
    pub figi: String,
}

impl MarketCommentaryMetadataDefinitionsListedSecurity {
    pub fn new(figi: String) -> MarketCommentaryMetadataDefinitionsListedSecurity {
        MarketCommentaryMetadataDefinitionsListedSecurity { figi }
    }
}
