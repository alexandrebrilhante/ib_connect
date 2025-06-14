use crate::models;
use serde::{Deserialize, Serialize};

/// Idea : Post an idea response to optional recipients,  along with an optional suggestion. The @type parameter needs to be \"IDEA\". Only 5 idea responses are accepted per eventId.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Idea {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "eventCorrelationId")]
    pub event_correlation_id: String,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<models::UserId>>,
    /// Suggestions must contain at least one of: either a text or textElements (but not both), a structuredTemplate, or structuredElements.
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<models::Suggestion>,
    #[serde(
        rename = "ideaHeaderProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub idea_header_properties: Option<Box<models::IdeaHeaderProperties>>,
}

impl Idea {
    /// Post an idea response to optional recipients,  along with an optional suggestion. The @type parameter needs to be \"IDEA\". Only 5 idea responses are accepted per eventId.
    pub fn new(
        at_type: AtType,
        event_correlation_id: String,
        suggestions: Vec<models::Suggestion>,
    ) -> Idea {
        Idea {
            at_type,
            event_correlation_id,
            recipients: None,
            suggestions,
            idea_header_properties: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "IDEA")]
    Idea,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Idea
    }
}
