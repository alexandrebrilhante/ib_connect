use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsIntent {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsIntentIntentPostIntent>>,
}

impl EnrichmentFieldsIntent {
    pub fn new(at_type: AtType) -> EnrichmentFieldsIntent {
        EnrichmentFieldsIntent {
            at_type,
            intent: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "intent")]
    Intent,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Intent
    }
}
