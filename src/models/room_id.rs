use crate::models;
use serde::{Deserialize, Serialize};

/// RoomId : An object that identifies an IB chat room. This can represent a transient or persistent chat room.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomId {
    #[serde(rename = "roomId")]
    pub room_id: String,
}

impl RoomId {
    /// An object that identifies an IB chat room. This can represent a transient or persistent chat room.
    pub fn new(room_id: String) -> RoomId {
        RoomId { room_id }
    }
}
