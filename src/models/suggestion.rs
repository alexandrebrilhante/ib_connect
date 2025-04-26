use crate::models;
use serde::{Deserialize, Serialize};

/// Suggestion : A Suggestion must contain at least one of: a text or textElement (but not both), a structuredTemplate, or structuredElements.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    /// Message 6000 characters or less in length. The text element is planned to be deprecated in favour of textElements. Cannot be used with the textElements property.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The type of action which can be taken on this suggestion. VIEW_ONLY means the suggestion cannot be shared with anyone else; therefore hiding any sharing capabilities. ORIGINAL_ROOM allows sharing of a suggestion into the chat room which contains the triggering message. For (solicited) ideas, ORIGINAL_ROOM is the default action, if no property is explicitly provided. For unsolicited ideas, the default action is to allow sharing of a suggestion to a chat room if VIEW_ONLY is not provided. ORIGINAL_ROOM does not apply to unsolicited ideas. If ORIGINAL_ROOM is provided for unsolicited ideas, it will be ignored and the default action of unsolicited ideas will be performed instead.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "structuredTemplate", skip_serializing_if = "Option::is_none")]
    pub structured_template: Option<Vec<models::StructuredTemplate>>,
    /// An array containing text or link elements that will be concatenated in the idea. Cannot be used with the text property.
    #[serde(rename = "textElements", skip_serializing_if = "Option::is_none")]
    pub text_elements: Option<Vec<models::IdeaTextElement>>,
    /// An array containing structured content elements that will be appended to the end of the message. See IdeaStructuredElement for supported types.
    #[serde(rename = "structuredElements", skip_serializing_if = "Option::is_none")]
    pub structured_elements: Option<Vec<models::IdeaStructuredElement>>,
}

impl Suggestion {
    /// A Suggestion must contain at least one of: a text or textElement (but not both), a structuredTemplate, or structuredElements.
    pub fn new() -> Suggestion {
        Suggestion {
            text: None,
            action: None,
            structured_template: None,
            text_elements: None,
            structured_elements: None,
        }
    }
}
/// The type of action which can be taken on this suggestion. VIEW_ONLY means the suggestion cannot be shared with anyone else; therefore hiding any sharing capabilities. ORIGINAL_ROOM allows sharing of a suggestion into the chat room which contains the triggering message. For (solicited) ideas, ORIGINAL_ROOM is the default action, if no property is explicitly provided. For unsolicited ideas, the default action is to allow sharing of a suggestion to a chat room if VIEW_ONLY is not provided. ORIGINAL_ROOM does not apply to unsolicited ideas. If ORIGINAL_ROOM is provided for unsolicited ideas, it will be ignored and the default action of unsolicited ideas will be performed instead.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "VIEW_ONLY")]
    ViewOnly,
    #[serde(rename = "ORIGINAL_ROOM")]
    OriginalRoom,
}

impl Default for Action {
    fn default() -> Action {
        Self::ViewOnly
    }
}
