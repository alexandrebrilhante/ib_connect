use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadStream200Response {
    ContentEvent(Box<models::ContentEvent>),
    IdeaDrawerFeedbackEvent(Box<models::IdeaDrawerFeedbackEvent>),
}

impl Default for ReadStream200Response {
    fn default() -> Self {
        Self::ContentEvent(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "IDEA_DRAWER_FEEDBACK_EVENT")]
    IdeaDrawerFeedbackEvent,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::IdeaDrawerFeedbackEvent
    }
}
/// Indicates whether the suggestion was sent as is or edited
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "SENT_SUGGESTION")]
    SentSuggestion,
    #[serde(rename = "EDITED_SUGGESTION")]
    EditedSuggestion,
}

impl Default for Action {
    fn default() -> Action {
        Self::SentSuggestion
    }
}
