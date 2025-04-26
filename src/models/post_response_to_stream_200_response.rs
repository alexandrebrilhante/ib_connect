use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostResponseToStream200Response {
    /// unique IDs for suggestions in idea, returned in the same order as sent
    #[serde(rename = "suggestionIds", skip_serializing_if = "Option::is_none")]
    pub suggestion_ids: Option<Vec<String>>,
}

impl PostResponseToStream200Response {
    pub fn new() -> PostResponseToStream200Response {
        PostResponseToStream200Response {
            suggestion_ids: None,
        }
    }
}
