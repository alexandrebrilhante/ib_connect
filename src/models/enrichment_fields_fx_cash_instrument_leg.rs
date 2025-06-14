use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxCashInstrumentLeg {
    /// Rate
    #[serde(rename = "all_in_rate", skip_serializing_if = "Option::is_none")]
    pub all_in_rate: Option<f64>,
    /// Quantity
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Direction of quote
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Quoted points for the leg
    #[serde(rename = "forward_points", skip_serializing_if = "Option::is_none")]
    pub forward_points: Option<f64>,
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
}

impl EnrichmentFieldsFxCashInstrumentLeg {
    pub fn new() -> EnrichmentFieldsFxCashInstrumentLeg {
        EnrichmentFieldsFxCashInstrumentLeg {
            all_in_rate: None,
            amount: None,
            direction: None,
            forward_points: None,
            settlement_date: None,
        }
    }
}
/// Direction of quote
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "BORROW")]
    Borrow,
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Borrow
    }
}
