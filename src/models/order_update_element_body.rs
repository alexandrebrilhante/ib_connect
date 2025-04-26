use crate::models;
use serde::{Deserialize, Serialize};

/// OrderUpdateElementBody : The body of the Order Update.
/// The body of the Order Update.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderUpdateElementBody {
    MarketOrder(Box<models::MarketOrder>),
    LimitOrder(Box<models::LimitOrder>),
}

impl Default for OrderUpdateElementBody {
    fn default() -> Self {
        Self::MarketOrder(Default::default())
    }
}
