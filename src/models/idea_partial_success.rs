use crate::models;
use serde::{Deserialize, Serialize};

/// IdeaPartialSuccess : Error Response Schema for 207 partial success
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdeaPartialSuccess {
    #[serde(rename = "errors")]
    pub errors: Vec<models::ErrorSchemaErrorsInner>,
    /// unique IDs for suggestions in idea, returned in the same order as sent
    #[serde(rename = "suggestionIds", skip_serializing_if = "Option::is_none")]
    pub suggestion_ids: Option<Vec<String>>,
}

impl IdeaPartialSuccess {
    /// Error Response Schema for 207 partial success
    pub fn new(errors: Vec<models::ErrorSchemaErrorsInner>) -> IdeaPartialSuccess {
        IdeaPartialSuccess {
            errors,
            suggestion_ids: None,
        }
    }
}
