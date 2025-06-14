use crate::models;
use serde::{Deserialize, Serialize};

/// FeedMessageElement : An element of a message that can be combined to form rich content messages.
/// An element of a message that can be combined to form rich content messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeedMessageElement {
    TextElement(Box<models::TextElement>),
    LinkElement(Box<models::LinkElement>),
    UserMentionElement(Box<models::UserMentionElement>),
    TableElement(Box<models::TableElement>),
    AttachmentElement(Box<models::AttachmentElement>),
}

impl Default for FeedMessageElement {
    fn default() -> Self {
        Self::TextElement(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "ATTACHMENT")]
    Attachment,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Attachment
    }
}
