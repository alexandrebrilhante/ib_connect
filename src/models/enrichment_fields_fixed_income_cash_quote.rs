use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFixedIncomeCashQuote {
    /// FIGI of benchmark security
    #[serde(rename = "benchmark", skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    /// Yellowkey of benchmark security
    #[serde(
        rename = "benchmark_yellow_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub benchmark_yellow_key: Option<BenchmarkYellowKey>,
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    /// How changeable the stated quote is
    #[serde(rename = "firmness", skip_serializing_if = "Option::is_none")]
    pub firmness: Option<Firmness>,
    /// Three letter Currency identifier
    #[serde(
        rename = "settlement_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub settlement_currency: Option<String>,
    /// The text scraped for settlement date.
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<String>,
    #[serde(rename = "underlying_equity", skip_serializing_if = "Option::is_none")]
    pub underlying_equity:
        Option<Box<models::EnrichmentFieldsFixedIncomeCashQuoteUnderlyingEquity>>,
    /// An array of different quote value types like price, yield, spread, discount margin
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::EnrichmentFieldsFixedIncomeCashQuoteValuesInner>>,
}

impl EnrichmentFieldsFixedIncomeCashQuote {
    pub fn new() -> EnrichmentFieldsFixedIncomeCashQuote {
        EnrichmentFieldsFixedIncomeCashQuote {
            benchmark: None,
            benchmark_yellow_key: None,
            confidence_type: None,
            firmness: None,
            settlement_currency: None,
            settlement_date: None,
            underlying_equity: None,
            values: None,
        }
    }
}
/// Yellowkey of benchmark security
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BenchmarkYellowKey {
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

impl Default for BenchmarkYellowKey {
    fn default() -> BenchmarkYellowKey {
        Self::Govt
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
/// How changeable the stated quote is
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Firmness {
    #[serde(rename = "FIRM")]
    Firm,
    #[serde(rename = "INDICATIVE")]
    Indicative,
}

impl Default for Firmness {
    fn default() -> Firmness {
        Self::Firm
    }
}
