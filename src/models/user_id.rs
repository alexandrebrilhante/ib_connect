use crate::models;
use serde::{Deserialize, Serialize};

/// UserId : An object that uniquely identifies a Bloomberg user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    #[serde(rename = "uuid")]
    pub uuid: i64,
}

impl UserId {
    /// An object that uniquely identifies a Bloomberg user.
    pub fn new(uuid: i64) -> UserId {
        UserId { uuid }
    }
}
