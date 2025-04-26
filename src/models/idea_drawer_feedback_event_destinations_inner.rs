use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdeaDrawerFeedbackEventDestinationsInner {
    RoomId(Box<models::RoomId>),
    UserId(Box<models::UserId>),
}

impl Default for IdeaDrawerFeedbackEventDestinationsInner {
    fn default() -> Self {
        Self::RoomId(Default::default())
    }
}
