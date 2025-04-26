use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryMetadataDefinitions {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<models::ListedSecurity>>,
    #[serde(rename = "listed-security", skip_serializing_if = "Option::is_none")]
    pub listed_security: Option<Box<models::MarketCommentaryMetadataDefinitionsListedSecurity>>,
}

impl MarketCommentaryMetadataDefinitions {
    pub fn new() -> MarketCommentaryMetadataDefinitions {
        MarketCommentaryMetadataDefinitions {
            instruments: None,
            listed_security: None,
        }
    }
}
