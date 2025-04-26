use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsSecurityLendingQuoteStartTermDate {
    /// Resolved date in MM/DD/YYYY format
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl StructuredTemplateFieldsSecurityLendingQuoteStartTermDate {
    pub fn new() -> StructuredTemplateFieldsSecurityLendingQuoteStartTermDate {
        StructuredTemplateFieldsSecurityLendingQuoteStartTermDate { value: None }
    }
}
