use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplateFieldsFixedIncomeCashSize : Generic group Size
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsFixedIncomeCashSize {
    /// Ask size (in security currency)
    #[serde(rename = "ask_size", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<f64>,
    /// Bid size (in security currency)
    #[serde(rename = "bid_size", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<f64>,
    /// The size of a deal
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

impl StructuredTemplateFieldsFixedIncomeCashSize {
    /// Generic group Size
    pub fn new() -> StructuredTemplateFieldsFixedIncomeCashSize {
        StructuredTemplateFieldsFixedIncomeCashSize {
            ask_size: None,
            bid_size: None,
            size: None,
        }
    }
}
