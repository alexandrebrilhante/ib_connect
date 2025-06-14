use crate::models;
use serde::{Deserialize, Serialize};

/// UideaDestinations : The destination list that the ideas can be shared with by the recipients. Only one of: roomIds or destinationIds can be provided.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UideaDestinations {
    /// The persistent room ID list that the ideas can be shared to.
    #[serde(rename = "roomIds", skip_serializing_if = "Option::is_none")]
    pub room_ids: Option<Vec<models::RoomId>>,
    /// Persistent room IDs or user UUIDs that the idea can be shared to.
    #[serde(rename = "destinationIds", skip_serializing_if = "Option::is_none")]
    pub destination_ids: Option<Vec<models::IdeaDrawerFeedbackEventDestinationsInner>>,
}

impl UideaDestinations {
    /// The destination list that the ideas can be shared with by the recipients. Only one of: roomIds or destinationIds can be provided.
    pub fn new() -> UideaDestinations {
        UideaDestinations {
            room_ids: None,
            destination_ids: None,
        }
    }
}
