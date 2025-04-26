use crate::models;
use serde::{Deserialize, Serialize};

/// EnrichmentFieldsFxSideLegsInner : Side information for a given leg
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsFxSideLegsInner {
    /// Free-form string to identify leg information in various sections
    #[serde(rename = "leg_id", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<String>,
    /// Payment stream direction for an asset (currency pair in a single leg or a far leg of the swap)
    #[serde(
        rename = "maker_base_direction",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_base_direction: Option<MakerBaseDirection>,
    /// Payment stream direction for an asset (currency pair in a single leg or a far leg of the swap)
    #[serde(
        rename = "taker_base_direction",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_base_direction: Option<TakerBaseDirection>,
}

impl EnrichmentFieldsFxSideLegsInner {
    /// Side information for a given leg
    pub fn new() -> EnrichmentFieldsFxSideLegsInner {
        EnrichmentFieldsFxSideLegsInner {
            leg_id: None,
            maker_base_direction: None,
            taker_base_direction: None,
        }
    }
}
/// Payment stream direction for an asset (currency pair in a single leg or a far leg of the swap)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MakerBaseDirection {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWO-WAY")]
    TwoWay,
}

impl Default for MakerBaseDirection {
    fn default() -> MakerBaseDirection {
        Self::Buy
    }
}
/// Payment stream direction for an asset (currency pair in a single leg or a far leg of the swap)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TakerBaseDirection {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWO-WAY")]
    TwoWay,
}

impl Default for TakerBaseDirection {
    fn default() -> TakerBaseDirection {
        Self::Buy
    }
}
