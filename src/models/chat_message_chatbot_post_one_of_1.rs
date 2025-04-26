use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageChatbotPostOneOf1 {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl ChatMessageChatbotPostOneOf1 {
    pub fn new() -> ChatMessageChatbotPostOneOf1 {
        ChatMessageChatbotPostOneOf1 { text: None }
    }
}
