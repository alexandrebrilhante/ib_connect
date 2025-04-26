use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsFixedIncomeCashQuoteUnderlyingEquity {
    /// Equity price associated with bond
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Ticker associated with the security
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

impl StructuredTemplateFieldsFixedIncomeCashQuoteUnderlyingEquity {
    pub fn new() -> StructuredTemplateFieldsFixedIncomeCashQuoteUnderlyingEquity {
        StructuredTemplateFieldsFixedIncomeCashQuoteUnderlyingEquity {
            price: None,
            ticker: None,
        }
    }
}
