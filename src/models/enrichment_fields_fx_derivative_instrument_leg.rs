use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxDerivativeInstrumentLeg {
    #[serde(rename = "delivery_date", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    /// Option delta
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    #[serde(rename = "notional", skip_serializing_if = "Option::is_none")]
    pub notional: Option<Box<models::EnrichmentFieldsCommonFieldsNotional>>,
    #[serde(rename = "premium", skip_serializing_if = "Option::is_none")]
    pub premium: Option<Box<models::EnrichmentFieldsCommonFieldsPremium>>,
    /// Put call code of the option
    #[serde(rename = "put_call_code", skip_serializing_if = "Option::is_none")]
    pub put_call_code: Option<PutCallCode>,
    /// Strike price of option
    #[serde(rename = "strike_price", skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<String>,
    #[serde(rename = "volatility", skip_serializing_if = "Option::is_none")]
    pub volatility: Option<Box<models::EnrichmentFieldsCommonFieldsVolatility>>,
}

impl EnrichmentFieldsFxDerivativeInstrumentLeg {
    pub fn new() -> EnrichmentFieldsFxDerivativeInstrumentLeg {
        EnrichmentFieldsFxDerivativeInstrumentLeg {
            delivery_date: None,
            delta: None,
            expiration_date: None,
            notional: None,
            premium: None,
            put_call_code: None,
            strike_price: None,
            volatility: None,
        }
    }
}
/// Put call code of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PutCallCode {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
}

impl Default for PutCallCode {
    fn default() -> PutCallCode {
        Self::Put
    }
}
