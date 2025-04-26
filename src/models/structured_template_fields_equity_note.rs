use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsEquityNote : Generic group Note
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquityNote {
    /// Free-form text for specifying additional details in SPW negotiations
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl StructuredTemplateFieldsEquityNote {
    /// Generic group Note
    pub fn new() -> StructuredTemplateFieldsEquityNote {
        StructuredTemplateFieldsEquityNote { text: None }
    }
}
