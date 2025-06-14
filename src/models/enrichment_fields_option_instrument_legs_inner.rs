use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsOptionInstrumentLegsInner {
    /// A leg type of the option
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsExpirationDate>>,
    /// Strike price of the option
    #[serde(rename = "strike_price", skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    /// A leg side of the option
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Ratio of an option leg
    #[serde(rename = "ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
    #[serde(rename = "linked_security", skip_serializing_if = "Option::is_none")]
    pub linked_security:
        Option<Box<models::EnrichmentFieldsOptionInstrumentLegsInnerLinkedSecurity>>,
}

impl EnrichmentFieldsOptionInstrumentLegsInner {
    pub fn new() -> EnrichmentFieldsOptionInstrumentLegsInner {
        EnrichmentFieldsOptionInstrumentLegsInner {
            r#type: None,
            expiration_date: None,
            strike_price: None,
            side: None,
            ratio: None,
            linked_security: None,
        }
    }
}
/// A leg type of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
}

impl Default for Type {
    fn default() -> Type {
        Self::Put
    }
}
/// A leg side of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}
