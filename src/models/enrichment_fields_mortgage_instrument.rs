use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsMortgageInstrument {
    /// Bloomberg Company Identifier
    #[serde(rename = "bbid", skip_serializing_if = "Option::is_none")]
    pub bbid: Option<String>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Coupon
    #[serde(rename = "coupon", skip_serializing_if = "Option::is_none")]
    pub coupon: Option<f64>,
    /// Current outstanding value
    #[serde(rename = "current_face", skip_serializing_if = "Option::is_none")]
    pub current_face: Option<f64>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi", skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    #[serde(rename = "maturity_date", skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_description: Option<String>,
    /// Ticker associated with the security
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Yellow key of security
    #[serde(rename = "yellow_key", skip_serializing_if = "Option::is_none")]
    pub yellow_key: Option<YellowKey>,
}

impl EnrichmentFieldsMortgageInstrument {
    pub fn new() -> EnrichmentFieldsMortgageInstrument {
        EnrichmentFieldsMortgageInstrument {
            bbid: None,
            confidence_type: None,
            coupon: None,
            current_face: None,
            figi: None,
            maturity_date: None,
            security_description: None,
            ticker: None,
            yellow_key: None,
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
/// Yellow key of security
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum YellowKey {
    #[serde(rename = "Govt")]
    Govt,
    #[serde(rename = "Corp")]
    Corp,
    #[serde(rename = "Mtge")]
    Mtge,
    #[serde(rename = "M-Mkt")]
    MMkt,
    #[serde(rename = "Muni")]
    Muni,
    #[serde(rename = "Pfd")]
    Pfd,
    #[serde(rename = "Equity")]
    Equity,
    #[serde(rename = "Comdty")]
    Comdty,
    #[serde(rename = "Index")]
    Index,
    #[serde(rename = "Curncy")]
    Curncy,
    #[serde(rename = "Client")]
    Client,
}

impl Default for YellowKey {
    fn default() -> YellowKey {
        Self::Govt
    }
}
