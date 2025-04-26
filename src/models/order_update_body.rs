use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateBody {
    #[serde(rename = "MarketOrder", skip_serializing_if = "Option::is_none")]
    pub market_order: Option<Box<models::MarketOrder>>,
    #[serde(rename = "LimitOrder", skip_serializing_if = "Option::is_none")]
    pub limit_order: Option<Box<models::LimitOrder>>,
    #[serde(rename = "OrderUpdateFields", skip_serializing_if = "Option::is_none")]
    pub order_update_fields: Option<Box<models::OrderUpdateBodyOrderUpdateFields>>,
}

impl OrderUpdateBody {
    pub fn new() -> OrderUpdateBody {
        OrderUpdateBody {
            market_order: None,
            limit_order: None,
            order_update_fields: None,
        }
    }
}
