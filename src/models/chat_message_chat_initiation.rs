use crate::models;
use serde::{Deserialize, Serialize};

/// ChatMessageChatInitiation : An object that contains the chat content that can be sent to IB.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageChatInitiation {
    #[serde(rename = "text")]
    pub text: String,
}

impl ChatMessageChatInitiation {
    /// An object that contains the chat content that can be sent to IB.
    pub fn new(text: String) -> ChatMessageChatInitiation {
        ChatMessageChatInitiation { text }
    }
}
