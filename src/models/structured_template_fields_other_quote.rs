use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOtherQuote {
    /// An array of different quote value types
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::StructuredTemplateFieldsOtherQuoteValuesInner>>,
}

impl StructuredTemplateFieldsOtherQuote {
    pub fn new() -> StructuredTemplateFieldsOtherQuote {
        StructuredTemplateFieldsOtherQuote { values: None }
    }
}
