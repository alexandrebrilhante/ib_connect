use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetractSuggestionRecipientsInner {
    UserId(Box<models::UserId>),
}

impl Default for RetractSuggestionRecipientsInner {
    fn default() -> Self {
        Self::UserId(Default::default())
    }
}
