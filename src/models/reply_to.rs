use crate::models;
use serde::{Deserialize, Serialize};

/// ReplyTo : This field gets populated when the event is sent as a reply to another event and contains information about the event that is being replied to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplyTo {
    /// The id of the IB event being replied to.
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// The structured content of the original message.
    #[serde(rename = "messageElements")]
    pub message_elements: Vec<models::FeedMessageElement>,
}

impl ReplyTo {
    /// This field gets populated when the event is sent as a reply to another event and contains information about the event that is being replied to.
    pub fn new(event_id: String, message_elements: Vec<models::FeedMessageElement>) -> ReplyTo {
        ReplyTo {
            event_id,
            message_elements,
        }
    }
}
