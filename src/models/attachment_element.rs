use crate::models;
use serde::{Deserialize, Serialize};

/// AttachmentElement : A generic attachment object attached to the IB message that is specified by type of attachment
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentElement {
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "attachment")]
    pub attachment: Box<models::AttachmentElementAttachment>,
}

impl AttachmentElement {
    /// A generic attachment object attached to the IB message that is specified by type of attachment
    pub fn new(
        content_type: ContentType,
        attachment: models::AttachmentElementAttachment,
    ) -> AttachmentElement {
        AttachmentElement {
            content_type,
            attachment: Box::new(attachment),
        }
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
