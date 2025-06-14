use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsFixedIncomeCashQuote {
    /// FIGI of benchmark security
    #[serde(rename = "benchmark", skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    /// Yellowkey of benchmark security
    #[serde(
        rename = "benchmark_yellow_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub benchmark_yellow_key: Option<BenchmarkYellowKey>,
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
        Option<Box<models::StructuredTemplateFieldsFixedIncomeCashQuoteUnderlyingEquity>>,
    /// An array of different quote value types like price, yield, spread, discount margin
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::StructuredTemplateFieldsFixedIncomeCashQuoteValuesInner>>,
}

impl StructuredTemplateFieldsFixedIncomeCashQuote {
    pub fn new() -> StructuredTemplateFieldsFixedIncomeCashQuote {
        StructuredTemplateFieldsFixedIncomeCashQuote {
            benchmark: None,
            benchmark_yellow_key: None,
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
