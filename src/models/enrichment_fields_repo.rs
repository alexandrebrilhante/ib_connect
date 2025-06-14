use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsRepo {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsRepoInstrument>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<models::EnrichmentFieldsRepoQuote>>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<models::EnrichmentFieldsRepoSide>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<models::EnrichmentFieldsRepoSizeSize>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsCreditDerivativeIntent>>,
}

impl EnrichmentFieldsRepo {
    pub fn new(at_type: AtType) -> EnrichmentFieldsRepo {
        EnrichmentFieldsRepo {
            at_type,
            instrument: None,
            quote: None,
            side: None,
            size: None,
            intent: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "repo")]
    Repo,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Repo
    }
}
