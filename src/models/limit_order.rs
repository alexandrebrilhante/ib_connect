use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitOrder {
    #[serde(rename = "side")]
    pub side: models::Side,
    #[serde(rename = "size")]
    pub size: models::Size,
    #[serde(rename = "instrument")]
    pub instrument: models::Instrument,
    /// The condition/rule applied to the order execution.
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "orderStatus")]
    pub order_status: models::Status,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<models::Strategy>,
    #[serde(rename = "timestamp")]
    pub timestamp: models::Timestamp,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<models::Notes>,
    #[serde(rename = "client")]
    pub client: models::Client,
    #[serde(rename = "broker")]
    pub broker: models::Broker,
    #[serde(rename = "filledQuantity")]
    pub filled_quantity: models::FilledQuantity,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<models::TimeInForce>,
    #[serde(rename = "trader", skip_serializing_if = "Option::is_none")]
    pub trader: Option<models::Trader>,
    #[serde(
        rename = "maxParticipationPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_participation_percentage: Option<models::MaxParticipationPercentage>,
    #[serde(rename = "benchmark", skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<models::Benchmark>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<models::Account>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<models::Venue>,
    #[serde(rename = "totalMarketValue", skip_serializing_if = "Option::is_none")]
    pub total_market_value: Option<models::TotalMarketValue>,
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<models::AveragePrice>,
    #[serde(rename = "limitPrice")]
    pub limit_price: models::LimitPrice,
}

impl LimitOrder {
    pub fn new(
        side: models::Side,
        size: models::Size,
        instrument: models::Instrument,
        order_type: OrderType,
        order_status: models::Status,
        timestamp: models::Timestamp,
        client: models::Client,
        broker: models::Broker,
        filled_quantity: models::FilledQuantity,
        limit_price: models::LimitPrice,
    ) -> LimitOrder {
        LimitOrder {
            side,
            size,
            instrument,
            order_type,
            order_status,
            strategy: None,
            timestamp,
            notes: None,
            client,
            broker,
            filled_quantity,
            time_in_force: None,
            trader: None,
            max_participation_percentage: None,
            benchmark: None,
            account: None,
            venue: None,
            total_market_value: None,
            average_price: None,
            limit_price,
        }
    }
}
/// The condition/rule applied to the order execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LMT")]
    Lmt,
}

impl Default for OrderType {
    fn default() -> OrderType {
        Self::Lmt
    }
}
