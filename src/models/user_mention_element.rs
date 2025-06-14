use crate::models;
use serde::{Deserialize, Serialize};

/// UserMentionElement : Mention Element (@John Doe)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserMentionElement {
    /// Should read \"USERMENTION\" for this element type
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "user")]
    pub user: Box<models::User>,
}

impl UserMentionElement {
    /// Mention Element (@John Doe)
    pub fn new(content_type: ContentType, user: models::User) -> UserMentionElement {
        UserMentionElement {
            content_type,
            user: Box::new(user),
        }
    }
}
/// Should read \"USERMENTION\" for this element type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "USERMENTION")]
    Usermention,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Usermention
    }
}
