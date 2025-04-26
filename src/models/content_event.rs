use crate::models;
use serde::{Deserialize, Serialize};

/// ContentEvent : Represents an event from the On Demand Feed, Base Feed, or Enriched Feed. The message parameter contains the actual content. The requester parameter will only be present in messages sent from the On Demand workflow. In this case, sender indicates the user who sent the message, while requester represents the user who forwarded the message to the API. Participants show a list of the participants in the chat room at the moment when this message is posted. Enrichments contain anything that a listening stream is interested in by this stream's configuration. For the Base Feed and Enriched Feed workflows, the 'messages' array will contain at most a single message. For the On Demand workflow, the'messages' array may contain multiple messages.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentEvent {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "roomId")]
    pub room_id: String,
    /// Name of the IB Chatroom. It is only provided for persistent chat rooms.
    #[serde(rename = "roomName", skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// Can be used to resume streaming from after this event when a disconnection happens
    #[serde(rename = "backfillId", skip_serializing_if = "Option::is_none")]
    pub backfill_id: Option<String>,
    #[serde(rename = "requester", skip_serializing_if = "Option::is_none")]
    pub requester: Option<Box<models::User>>,
    #[serde(rename = "participants")]
    pub participants: Vec<models::User>,
    #[serde(rename = "trigger")]
    pub trigger: models::Trigger,
    #[serde(rename = "enrichments", skip_serializing_if = "Option::is_none")]
    pub enrichments: Option<Vec<models::Enrichment>>,
    #[serde(rename = "messages")]
    pub messages: Vec<models::Message>,
}

impl ContentEvent {
    /// Represents an event from the On Demand Feed, Base Feed, or Enriched Feed. The message parameter contains the actual content. The requester parameter will only be present in messages sent from the On Demand workflow. In this case, sender indicates the user who sent the message, while requester represents the user who forwarded the message to the API. Participants show a list of the participants in the chat room at the moment when this message is posted. Enrichments contain anything that a listening stream is interested in by this stream's configuration. For the Base Feed and Enriched Feed workflows, the 'messages' array will contain at most a single message. For the On Demand workflow, the'messages' array may contain multiple messages.
    pub fn new(
        at_type: AtType,
        room_id: String,
        event_id: String,
        participants: Vec<models::User>,
        trigger: models::Trigger,
        messages: Vec<models::Message>,
    ) -> ContentEvent {
        ContentEvent {
            at_type,
            room_id,
            room_name: None,
            event_id,
            backfill_id: None,
            requester: None,
            participants,
            trigger,
            enrichments: None,
            messages,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "CONTENT_EVENT")]
    ContentEvent,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::ContentEvent
    }
}
