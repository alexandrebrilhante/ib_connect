use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorSchema : Error Response Schema per JSONApi Spec.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorSchema {
    #[serde(rename = "errors")]
    pub errors: Vec<models::ErrorSchemaErrorsInner>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl ErrorSchema {
    /// Error Response Schema per JSONApi Spec.
    pub fn new(errors: Vec<models::ErrorSchemaErrorsInner>) -> ErrorSchema {
        ErrorSchema {
            errors,
            error: None,
            error_description: None,
        }
    }
}
