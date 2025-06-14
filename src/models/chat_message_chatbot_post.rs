use crate::models;
use serde::{Deserialize, Serialize};

/// ChatMessageChatbotPost : An object that contains the chat content that can be sent to IB.
/// An object that contains the chat content that can be sent to IB.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageChatbotPost {
    ChatMessageChatbotPostOneOf(Box<models::ChatMessageChatbotPostOneOf>),
    ChatMessageChatbotPostOneOf1(Box<models::ChatMessageChatbotPostOneOf1>),
}

impl Default for ChatMessageChatbotPost {
    fn default() -> Self {
        Self::ChatMessageChatbotPostOneOf(Default::default())
    }
}
