use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsOptionInstrument {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying:
        Option<Box<models::StructuredTemplateFieldsOptionInstrumentInstrumentUnderlying>>,
    /// An options or an option strategy
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    /// An option style
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::StructuredTemplateFieldsOptionInstrumentLegsInner>>,
}

impl StructuredTemplateFieldsOptionInstrument {
    pub fn new() -> StructuredTemplateFieldsOptionInstrument {
        StructuredTemplateFieldsOptionInstrument {
            underlying: None,
            strategy: None,
            style: None,
            legs: None,
        }
    }
}
/// An options or an option strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "CALL_CALENDAR_SPREAD")]
    CallCalendarSpread,
    #[serde(rename = "CALL_DIAGONAL_CALENDAR_SPREAD")]
    CallDiagonalCalendarSpread,
    #[serde(rename = "CALL_SPREAD")]
    CallSpread,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUT_CALENDAR_SPREAD")]
    PutCalendarSpread,
    #[serde(rename = "PUT_DIAGONAL_CALENDAR_SPREAD")]
    PutDiagonalCalendarSpread,
    #[serde(rename = "PUT_SPREAD")]
    PutSpread,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "RISK_REVERSAL")]
    RiskReversal,
    #[serde(rename = "STRADDLE")]
    Straddle,
    #[serde(rename = "STRANGLE")]
    Strangle,
    #[serde(rename = "SYNTHETIC")]
    Synthetic,
    #[serde(rename = "CALL_FLY")]
    CallFly,
    #[serde(rename = "PUT_FLY")]
    PutFly,
    #[serde(rename = "CALL_RATIO")]
    CallRatio,
    #[serde(rename = "PUT_RATIO")]
    PutRatio,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "REVERSE_CONVERSION")]
    ReverseConversion,
    #[serde(rename = "JELLY_ROLL")]
    JellyRoll,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::CallCalendarSpread
    }
}
/// An option style
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "AMERICAN")]
    American,
    #[serde(rename = "EUROPEAN")]
    European,
}

impl Default for Style {
    fn default() -> Style {
        Self::American
    }
}
