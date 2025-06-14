use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxDerivativeInstrumentInstrument {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Currency pair
    #[serde(rename = "currency_pair", skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    #[serde(
        rename = "delta_hedge_notional",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_hedge_notional: Option<Box<models::EnrichmentFieldsCommonFieldsDeltaHedgeNotional>>,
    /// Type of FX derivative option
    #[serde(rename = "derivative_type", skip_serializing_if = "Option::is_none")]
    pub derivative_type: Option<DerivativeType>,
    /// Option expiry cut-off
    #[serde(rename = "expiration_cut", skip_serializing_if = "Option::is_none")]
    pub expiration_cut: Option<String>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi_underlying", skip_serializing_if = "Option::is_none")]
    pub figi_underlying: Option<String>,
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::FxDerivativeInstrumentLeg>>,
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<Box<models::EnrichmentFieldsCommonFieldsUnderlying>>,
}

impl EnrichmentFieldsFxDerivativeInstrumentInstrument {
    pub fn new() -> EnrichmentFieldsFxDerivativeInstrumentInstrument {
        EnrichmentFieldsFxDerivativeInstrumentInstrument {
            confidence_type: None,
            currency_pair: None,
            delta_hedge_notional: None,
            derivative_type: None,
            expiration_cut: None,
            figi_underlying: None,
            legs: None,
            underlying: None,
        }
    }
}
/// Confidence level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfidenceType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "MACHINE")]
    Machine,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ConfidenceType {
    fn default() -> ConfidenceType {
        Self::User
    }
}
/// Type of FX derivative option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DerivativeType {
    #[serde(rename = "VANILLA")]
    Vanilla,
}

impl Default for DerivativeType {
    fn default() -> DerivativeType {
        Self::Vanilla
    }
}
