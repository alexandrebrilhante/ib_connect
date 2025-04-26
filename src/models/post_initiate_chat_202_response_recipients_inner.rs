use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostInitiateChat202ResponseRecipientsInner {
    UserId(Box<models::UserId>),
    RoomId(Box<models::RoomId>),
}

impl Default for PostInitiateChat202ResponseRecipientsInner {
    fn default() -> Self {
        Self::UserId(Default::default())
    }
}
