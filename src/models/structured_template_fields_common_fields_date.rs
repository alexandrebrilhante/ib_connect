use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsCommonFieldsDate {
    /// Exact text as it appeared
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Resolved date in MM/DD/YYYY format
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl StructuredTemplateFieldsCommonFieldsDate {
    pub fn new() -> StructuredTemplateFieldsCommonFieldsDate {
        StructuredTemplateFieldsCommonFieldsDate {
            text: None,
            value: None,
        }
    }
}
