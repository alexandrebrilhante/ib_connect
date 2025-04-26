use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsSecurityLendingSide : Generic group Side. Most of the types use this one
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsSecurityLendingSide {
    /// Side of the quote or security mention
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

impl StructuredTemplateFieldsSecurityLendingSide {
    /// Generic group Side. Most of the types use this one
    pub fn new() -> StructuredTemplateFieldsSecurityLendingSide {
        StructuredTemplateFieldsSecurityLendingSide { side: None }
    }
}
/// Side of the quote or security mention
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORROW")]
    Borrow,
}

impl Default for Side {
    fn default() -> Side {
        Self::Lend
    }
}
