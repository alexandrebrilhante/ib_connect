use crate::models;
use serde::{Deserialize, Serialize};

/// OrderUpdateBodyOrderUpdateFieldsInstrument : The traded instrument.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateBodyOrderUpdateFieldsInstrument {
    /// Primary security FIGI identifier
    #[serde(rename = "figi")]
    pub figi: String,
}

impl OrderUpdateBodyOrderUpdateFieldsInstrument {
    /// The traded instrument.
    pub fn new(figi: String) -> OrderUpdateBodyOrderUpdateFieldsInstrument {
        OrderUpdateBodyOrderUpdateFieldsInstrument { figi }
    }
}
