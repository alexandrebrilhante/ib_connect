use crate::models;
use serde::{Deserialize, Serialize};

/// ChatbotPostRequest : Chatbot Post Request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatbotPostRequest {
    #[serde(rename = "chatbotId")]
    pub chatbot_id: Box<models::UserId>,
    #[serde(rename = "recipient")]
    pub recipient: Box<models::ChatbotPostRequestRecipient>,
    #[serde(rename = "chatMessage")]
    pub chat_message: Vec<models::ChatMessageChatbotPost>,
}

impl ChatbotPostRequest {
    /// Chatbot Post Request
    pub fn new(
        chatbot_id: models::UserId,
        recipient: models::ChatbotPostRequestRecipient,
        chat_message: Vec<models::ChatMessageChatbotPost>,
    ) -> ChatbotPostRequest {
        ChatbotPostRequest {
            chatbot_id: Box::new(chatbot_id),
            recipient: Box::new(recipient),
            chat_message,
        }
    }
}
