use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFixedIncomeCashInstrument {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Coupon
    #[serde(rename = "coupon", skip_serializing_if = "Option::is_none")]
    pub coupon: Option<f64>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi", skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    /// Company id of loan
    #[serde(rename = "loan_company_id", skip_serializing_if = "Option::is_none")]
    pub loan_company_id: Option<String>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_description: Option<String>,
    /// Series of security. e.g. 'REGS', '144A', '22', etc.
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    /// Ticker associated with the security
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Yellow key of security
    #[serde(rename = "yellow_key", skip_serializing_if = "Option::is_none")]
    pub yellow_key: Option<YellowKey>,
}

impl EnrichmentFieldsFixedIncomeCashInstrument {
    pub fn new() -> EnrichmentFieldsFixedIncomeCashInstrument {
        EnrichmentFieldsFixedIncomeCashInstrument {
            confidence_type: None,
            coupon: None,
            figi: None,
            loan_company_id: None,
            security_description: None,
            series: None,
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
