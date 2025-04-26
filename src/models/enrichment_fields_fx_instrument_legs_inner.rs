use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxInstrumentLegsInner : Instrument leg information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxInstrumentLegsInner {
    /// Free-form string to identify leg information in various sections
    #[serde(rename = "leg_id", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<String>,
    /// Used in FX to characterise the leg - single for spots and outrights, near or far leg in an FX swap
    #[serde(rename = "leg_type", skip_serializing_if = "Option::is_none")]
    pub leg_type: Option<LegType>,
    /// Indication when trade happens in onshore or offshore market
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<Market>,
    /// Indication of currencies being 'delivered' (exchanged on settlement) or trade being settled in cash
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
    #[serde(rename = "settlement_date", skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<Box<models::EnrichmentFieldsFxInstrumentLegsInnerSettlementDate>>,
    #[serde(rename = "fixing_date", skip_serializing_if = "Option::is_none")]
    pub fixing_date: Option<Box<models::EnrichmentFieldsFxInstrumentLegsInnerSettlementDate>>,
}

impl EnrichmentFieldsFxInstrumentLegsInner {
    /// Instrument leg information
    pub fn new() -> EnrichmentFieldsFxInstrumentLegsInner {
        EnrichmentFieldsFxInstrumentLegsInner {
            leg_id: None,
            leg_type: None,
            market: None,
            delivery: None,
            settlement_date: None,
            fixing_date: None,
        }
    }
}
/// Used in FX to characterise the leg - single for spots and outrights, near or far leg in an FX swap
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LegType {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "NEAR")]
    Near,
    #[serde(rename = "FAR")]
    Far,
}

impl Default for LegType {
    fn default() -> LegType {
        Self::Single
    }
}
/// Indication when trade happens in onshore or offshore market
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Market {
    #[serde(rename = "OFFSHORE")]
    Offshore,
    #[serde(rename = "ONSHORE")]
    Onshore,
}

impl Default for Market {
    fn default() -> Market {
        Self::Offshore
    }
}
/// Indication of currencies being 'delivered' (exchanged on settlement) or trade being settled in cash
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Delivery {
    #[serde(rename = "DELIVERABLE")]
    Deliverable,
    #[serde(rename = "NON-DELIVERABLE")]
    NonDeliverable,
}

impl Default for Delivery {
    fn default() -> Delivery {
        Self::Deliverable
    }
}
