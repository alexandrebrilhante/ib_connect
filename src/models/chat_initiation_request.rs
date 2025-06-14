use crate::models;
use serde::{Deserialize, Serialize};

/// ChatInitiationRequest : Chat Initiation Request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatInitiationRequest {
    #[serde(rename = "sender")]
    pub sender: Box<models::UserId>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<models::PostInitiateChat202ResponseRecipientsInner>>,
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<Box<models::JsonTable>>,
    #[serde(rename = "chatMessage", skip_serializing_if = "Option::is_none")]
    pub chat_message: Option<Box<models::ChatMessageChatInitiation>>,
    #[serde(rename = "structuredTemplate", skip_serializing_if = "Option::is_none")]
    pub structured_template: Option<Vec<models::StructuredTemplate>>,
}

impl ChatInitiationRequest {
    /// Chat Initiation Request
    pub fn new(sender: models::UserId) -> ChatInitiationRequest {
        ChatInitiationRequest {
            sender: Box::new(sender),
            recipients: None,
            table: None,
            chat_message: None,
            structured_template: None,
        }
    }
}
