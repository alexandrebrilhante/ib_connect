use crate::models;
use serde::{Deserialize, Serialize};

/// RetractSuggestion : Retraction of a Single Suggestion Element.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetractSuggestion {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    /// A unique identifier for a suggestion
    #[serde(rename = "suggestionId")]
    pub suggestion_id: String,
    /// List of users that the suggestion will be retracted from.
    #[serde(rename = "recipients")]
    pub recipients: Vec<models::RetractSuggestionRecipientsInner>,
}

impl RetractSuggestion {
    /// Retraction of a Single Suggestion Element.
    pub fn new(
        at_type: AtType,
        suggestion_id: String,
        recipients: Vec<models::RetractSuggestionRecipientsInner>,
    ) -> RetractSuggestion {
        RetractSuggestion {
            at_type,
            suggestion_id,
            recipients,
        }
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
