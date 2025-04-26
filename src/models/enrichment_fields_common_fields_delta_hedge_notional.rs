use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCommonFieldsDeltaHedgeNotional {
    /// Option delta hedge notional with respect to LHS currency
    #[serde(rename = "lhs", skip_serializing_if = "Option::is_none")]
    pub lhs: Option<f64>,
    /// Option delta hedge notional with respect to RHS currency
    #[serde(rename = "rhs", skip_serializing_if = "Option::is_none")]
    pub rhs: Option<f64>,
}

impl EnrichmentFieldsCommonFieldsDeltaHedgeNotional {
    pub fn new() -> EnrichmentFieldsCommonFieldsDeltaHedgeNotional {
        EnrichmentFieldsCommonFieldsDeltaHedgeNotional {
            lhs: None,
            rhs: None,
        }
    }
}
