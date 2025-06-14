use crate::models;
use serde::{Deserialize, Serialize};

/// Uidea : Post an unsolicited idea. Unlike IDEA, no prior eventCorrelationId is needed. The @type parameter needs to be \"UIDEA\". There is a max of 3 suggestions per request and 25 recipients.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Uidea {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    /// The users whose idea drawers will receive these ideas.
    #[serde(rename = "recipients")]
    pub recipients: Vec<models::RetractSuggestionRecipientsInner>,
    #[serde(rename = "destinations")]
    pub destinations: Box<models::UideaDestinations>,
    /// Suggestions must contain at least one of: either a text or textElements (but not both), a structuredTemplate, or structuredElements.
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<models::Suggestion>,
    #[serde(
        rename = "ideaHeaderProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub idea_header_properties: Option<Box<models::IdeaHeaderProperties>>,
}

impl Uidea {
    /// Post an unsolicited idea. Unlike IDEA, no prior eventCorrelationId is needed. The @type parameter needs to be \"UIDEA\". There is a max of 3 suggestions per request and 25 recipients.
    pub fn new(
        at_type: AtType,
        recipients: Vec<models::RetractSuggestionRecipientsInner>,
        destinations: models::UideaDestinations,
        suggestions: Vec<models::Suggestion>,
    ) -> Uidea {
        Uidea {
            at_type,
            recipients,
            destinations: Box::new(destinations),
            suggestions,
            idea_header_properties: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "UIDEA")]
    Uidea,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::Uidea
    }
}
