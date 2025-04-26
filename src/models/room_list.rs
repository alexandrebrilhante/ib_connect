use crate::models;
use serde::{Deserialize, Serialize};

/// RoomList : A list of objects each containing information about an IB Chatroom.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomList {
    #[serde(rename = "rooms")]
    pub rooms: Vec<models::Room>,
}

impl RoomList {
    /// A list of objects each containing information about an IB Chatroom.
    pub fn new(rooms: Vec<models::Room>) -> RoomList {
        RoomList { rooms }
    }
}
