use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateBodyOrderUpdateFields {
    /// The trade side.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// The quantity of the traded instrument.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::OrderUpdateBodyOrderUpdateFieldsInstrument>>,
    /// The status or condition of the order.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The name of the algorithm or the execution strategy.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// The epoch timestamp in milliseconds of the order update snapshot.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// Free form input for comments/special order instructions.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The client whose trade is being updated upon.
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    /// The broker involved in the order.
    #[serde(rename = "broker", skip_serializing_if = "Option::is_none")]
    pub broker: Option<String>,
    /// How much of the trade is done.
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<f64>,
    /// How long the order will remain active before it is executed or expires.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// The trader involved in the order.
    #[serde(rename = "trader", skip_serializing_if = "Option::is_none")]
    pub trader: Option<String>,
    /// How much of the average daily volume in that name are you willing to be accountable for?.
    #[serde(
        rename = "maxParticipationPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_participation_percentage: Option<String>,
    /// Reference instrument, basket, index, price or benchmark bond.
    #[serde(rename = "benchmark", skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    /// Trading account/client portfolio ID.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Exchange, ATS, dark pool - where the trade is being executed.
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,
    /// Approximate value of total order, or gross risk for undefined quantity of instrument.
    #[serde(rename = "totalMarketValue", skip_serializing_if = "Option::is_none")]
    pub total_market_value: Option<f64>,
    /// Weighted average price of executed volume
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f64>,
    /// The limit price for the order. Do not execute higher (buy) or lower (sell) than this
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    /// Target execution price
    #[serde(rename = "targetPrice", skip_serializing_if = "Option::is_none")]
    pub target_price: Option<f64>,
}

impl OrderUpdateBodyOrderUpdateFields {
    pub fn new() -> OrderUpdateBodyOrderUpdateFields {
        OrderUpdateBodyOrderUpdateFields {
            side: None,
            size: None,
            instrument: None,
            status: None,
            strategy: None,
            timestamp: None,
            notes: None,
            client: None,
            broker: None,
            filled_quantity: None,
            time_in_force: None,
            trader: None,
            max_participation_percentage: None,
            benchmark: None,
            account: None,
            venue: None,
            total_market_value: None,
            average_price: None,
            limit_price: None,
            target_price: None,
        }
    }
}
/// The status or condition of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Sent")]
    Sent,
    #[serde(rename = "Working")]
    Working,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Part-filled")]
    PartFilled,
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "CxlPend")]
    CxlPend,
    #[serde(rename = "Cancel")]
    Cancel,
    #[serde(rename = "CxlRej")]
    CxlRej,
    #[serde(rename = "CxlForce")]
    CxlForce,
    #[serde(rename = "CxlRep")]
    CxlRep,
    #[serde(rename = "CxlRepRej")]
    CxlRepRej,
    #[serde(rename = "Replaced")]
    Replaced,
    #[serde(rename = "CxlRepReq")]
    CxlRepReq,
    #[serde(rename = "Done")]
    Done,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Hold")]
    Hold,
    #[serde(rename = "Corrected")]
    Corrected,
    #[serde(rename = "Allocation Sent")]
    AllocationSent,
    #[serde(rename = "Allocated")]
    Allocated,
    #[serde(rename = "New")]
    New,
    #[serde(rename = "Assign")]
    Assign,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "Modified")]
    Modified,
    #[serde(rename = "Split Sent")]
    SplitSent,
    #[serde(rename = "Routed")]
    Routed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Sent
    }
}
/// How long the order will remain active before it is executed or expires.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "OPG")]
    Opg,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "GTD")]
    Gtd,
    #[serde(rename = "GTX")]
    Gtx,
}

impl Default for TimeInForce {
    fn default() -> TimeInForce {
        Self::Day
    }
}
