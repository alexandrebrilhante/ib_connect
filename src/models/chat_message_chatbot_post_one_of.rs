use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageChatbotPostOneOf {
    /// An array of different types of elements. Restrictions apply: each message can contain up to 10 MentionElements, 1 TableElement or 1 CardElement.
    #[serde(rename = "messageElements", skip_serializing_if = "Option::is_none")]
    pub message_elements: Option<Vec<models::MessageElement>>,
}

impl ChatMessageChatbotPostOneOf {
    pub fn new() -> ChatMessageChatbotPostOneOf {
        ChatMessageChatbotPostOneOf {
            message_elements: None,
        }
    }
}
