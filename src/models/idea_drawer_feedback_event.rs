use crate::models;
use serde::{Deserialize, Serialize};

/// IdeaDrawerFeedbackEvent : Represents an idea drawer feedback event from either the manual or automatic workflow. This event is sent back when a suggestion has been acted upon in idea drawer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdeaDrawerFeedbackEvent {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    /// A unique identifier for a suggestion
    #[serde(rename = "suggestionId")]
    pub suggestion_id: String,
    /// The room ids that the suggestion was shared with
    #[serde(rename = "destinations", skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<models::IdeaDrawerFeedbackEventDestinationsInner>>,
    #[serde(rename = "senderId", skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<Box<models::UserId>>,
    /// Indicates whether the suggestion was sent as is or edited
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Can be used to resume streaming from after this event when a disconnection happens
    #[serde(rename = "backfillId", skip_serializing_if = "Option::is_none")]
    pub backfill_id: Option<String>,
}

impl IdeaDrawerFeedbackEvent {
    /// Represents an idea drawer feedback event from either the manual or automatic workflow. This event is sent back when a suggestion has been acted upon in idea drawer.
    pub fn new(at_type: AtType, suggestion_id: String) -> IdeaDrawerFeedbackEvent {
        IdeaDrawerFeedbackEvent {
            at_type,
            suggestion_id,
            destinations: None,
            sender_id: None,
            action: None,
            backfill_id: None,
        }
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
