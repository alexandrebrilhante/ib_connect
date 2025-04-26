use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsMortgageQuote {
    /// How changeable the stated quote is
    #[serde(rename = "firmness", skip_serializing_if = "Option::is_none")]
    pub firmness: Option<Firmness>,
    /// Reference price of underlying asset
    #[serde(rename = "reference_price", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<f64>,
    /// The text scraped for settlement date.
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::StructuredTemplateFieldsMortgageQuoteValuesInner>>,
}

impl StructuredTemplateFieldsMortgageQuote {
    pub fn new() -> StructuredTemplateFieldsMortgageQuote {
        StructuredTemplateFieldsMortgageQuote {
            firmness: None,
            reference_price: None,
            settlement_date: None,
            values: None,
        }
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
