use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchResponseToStreamRequest {
    RetractSuggestion(Box<models::RetractSuggestion>),
}

impl Default for PatchResponseToStreamRequest {
    fn default() -> Self {
        Self::RetractSuggestion(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "RETRACT_SUGGESTION")]
    RetractSuggestion,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::RetractSuggestion
    }
}
