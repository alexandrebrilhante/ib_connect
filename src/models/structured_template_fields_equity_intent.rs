use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsEquityIntent : Generic group Intent
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsEquityIntent {
    /// Name of the security-level intent
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}

impl StructuredTemplateFieldsEquityIntent {
    /// Generic group Intent
    pub fn new() -> StructuredTemplateFieldsEquityIntent {
        StructuredTemplateFieldsEquityIntent { name: None }
    }
}
/// Name of the security-level intent
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "Inquiry")]
    Inquiry,
    #[serde(rename = "Quote")]
    Quote,
}

impl Default for Name {
    fn default() -> Name {
        Self::Inquiry
    }
}
