use crate::models;
use serde::{Deserialize, Serialize};

/// Room : An object containing information about an IB room.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "roomId")]
    pub room_id: String,
}

impl Room {
    /// An object containing information about an IB room.
    pub fn new(room_id: String) -> Room {
        Room { room_id }
    }
}
