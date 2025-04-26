use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsMortgageQuote {
    /// Confidence level
    #[serde(rename = "confidence_type", skip_serializing_if = "Option::is_none")]
    pub confidence_type: Option<ConfidenceType>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Box<models::StructuredTemplateFieldsSecurityLendingQuoteStartTermDate>>,
    /// How changeable the stated quote is
    #[serde(rename = "firmness", skip_serializing_if = "Option::is_none")]
    pub firmness: Option<Firmness>,
    /// Premium for security over the base price
    #[serde(rename = "payup", skip_serializing_if = "Option::is_none")]
    pub payup: Option<f64>,
    /// Rate at which loan can be prepaid
    #[serde(rename = "prepay", skip_serializing_if = "Option::is_none")]
    pub prepay: Option<String>,
    /// Reference price of underlying asset
    #[serde(rename = "reference_price", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<f64>,
    /// The text scraped for settlement date.
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::EnrichmentFieldsMortgageQuoteValuesInner>>,
}

impl EnrichmentFieldsMortgageQuote {
    pub fn new() -> EnrichmentFieldsMortgageQuote {
        EnrichmentFieldsMortgageQuote {
            confidence_type: None,
            due_date: None,
            firmness: None,
            payup: None,
            prepay: None,
            reference_price: None,
            settlement_date: None,
            values: None,
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
