use crate::models;
use serde::{Deserialize, Serialize};

/// Message : An object providing message information. Message is the raw message content. It is an optional field dependent on the triggering event type. For example a manually triggered event would include this field. Timestamp is when this message is posted in the chat room. Sender is the one who posts this message. messageElements is an array of structured content; for example if a message is composed of a regular text message (e.g. \"Hello, check out the table below:\") followed by a table, messageElements will contain two items 1) a TextElement representing the regular text 2) a TableElement representing the table portition of the raw message
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "messageElements", skip_serializing_if = "Option::is_none")]
    pub message_elements: Option<Vec<models::FeedMessageElement>>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "sender")]
    pub sender: Box<models::User>,
    #[serde(rename = "blastInfo", skip_serializing_if = "Option::is_none")]
    pub blast_info: Option<Box<models::BlastInfo>>,
    /// Unique identifier of the IB post. For Base Feed or Enriched Feed, this value will be the same as eventId. For On Demand Feed each IB post included in the event has a separate post Id.
    #[serde(rename = "ibPostId", skip_serializing_if = "Option::is_none")]
    pub ib_post_id: Option<String>,
    /// Timestamp in microseconds.
    #[serde(rename = "microTimestamp", skip_serializing_if = "Option::is_none")]
    pub micro_timestamp: Option<i64>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Box<models::ReplyTo>>,
}

impl Message {
    /// An object providing message information. Message is the raw message content. It is an optional field dependent on the triggering event type. For example a manually triggered event would include this field. Timestamp is when this message is posted in the chat room. Sender is the one who posts this message. messageElements is an array of structured content; for example if a message is composed of a regular text message (e.g. \"Hello, check out the table below:\") followed by a table, messageElements will contain two items 1) a TextElement representing the regular text 2) a TableElement representing the table portition of the raw message
    pub fn new(timestamp: String, sender: models::User) -> Message {
        Message {
            message: None,
            message_elements: None,
            timestamp,
            sender: Box::new(sender),
            blast_info: None,
            ib_post_id: None,
            micro_timestamp: None,
            reply_to: None,
        }
    }
}
