use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatbotPostRequestRecipient {
    RoomId(Box<models::RoomId>),
}

impl Default for ChatbotPostRequestRecipient {
    fn default() -> Self {
        Self::RoomId(Default::default())
    }
}
