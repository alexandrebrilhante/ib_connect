use crate::models;
use serde::{Deserialize, Serialize};

/// MentionElement : Mention Element (@John Doe) - Max of 10 per message
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MentionElement {
    /// Should read \"MENTION\" for this element type
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "user")]
    pub user: Box<models::UserId>,
}

impl MentionElement {
    /// Mention Element (@John Doe) - Max of 10 per message
    pub fn new(content_type: ContentType, user: models::UserId) -> MentionElement {
        MentionElement {
            content_type,
            user: Box::new(user),
        }
    }
}
/// Should read \"MENTION\" for this element type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "MENTION")]
    Mention,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Mention
    }
}
