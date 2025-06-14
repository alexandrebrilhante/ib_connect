use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsCreditDerivativeInstrument {
    /// Brand name for credit default swap index
    #[serde(rename = "cdx_group", skip_serializing_if = "Option::is_none")]
    pub cdx_group: Option<String>,
    #[serde(rename = "cdx_offer_date", skip_serializing_if = "Option::is_none")]
    pub cdx_offer_date:
        Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTermDate>>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// Coupon
    #[serde(rename = "coupon", skip_serializing_if = "Option::is_none")]
    pub coupon: Option<f64>,
    /// Primary security FIGI identifier
    #[serde(rename = "figi_underlying", skip_serializing_if = "Option::is_none")]
    pub figi_underlying: Option<String>,
    /// Series rolling from (e.g. for CDX rolls)
    #[serde(rename = "roll_from_series", skip_serializing_if = "Option::is_none")]
    pub roll_from_series: Option<f64>,
    /// Sector of security.  Currently one of: HY (high yield) or IG (investment grade)
    #[serde(rename = "sector", skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_description: Option<String>,
    /// Series of security. e.g. 'REGS', '144A', '22', etc.
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    /// Tenor associated with security.
    #[serde(rename = "tenor", skip_serializing_if = "Option::is_none")]
    pub tenor: Option<String>,
    /// Ticker associated with the security
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Year definition of the security (e.g. CDS)
    #[serde(rename = "year_definition", skip_serializing_if = "Option::is_none")]
    pub year_definition: Option<f64>,
}

impl EnrichmentFieldsCreditDerivativeInstrument {
    pub fn new() -> EnrichmentFieldsCreditDerivativeInstrument {
        EnrichmentFieldsCreditDerivativeInstrument {
            cdx_group: None,
            cdx_offer_date: None,
            confidence_type: None,
            coupon: None,
            figi_underlying: None,
            roll_from_series: None,
            sector: None,
            security_description: None,
            series: None,
            tenor: None,
            ticker: None,
            year_definition: None,
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
