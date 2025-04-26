use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsSecurityLendingQuoteStartTerm {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTermDate>>,
    /// The term number of days. When used as a start term, it is number of days after the contract is signed. As an end term, it is a duration of the contract.
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

impl StructuredTemplateFieldsSecurityLendingQuoteStartTerm {
    pub fn new() -> StructuredTemplateFieldsSecurityLendingQuoteStartTerm {
        StructuredTemplateFieldsSecurityLendingQuoteStartTerm {
            date: None,
            days: None,
        }
    }
}
