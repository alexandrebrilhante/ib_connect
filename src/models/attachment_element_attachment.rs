use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachmentElementAttachment {
    FileAttachmentElement(Box<models::FileAttachmentElement>),
    ComponentAttachmentElement(Box<models::ComponentAttachmentElement>),
    AudioAttachmentElement(Box<models::AudioAttachmentElement>),
}

impl Default for AttachmentElementAttachment {
    fn default() -> Self {
        Self::FileAttachmentElement(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "COMPONENT")]
    Component,
    #[serde(rename = "AUDIO")]
    Audio,
}

impl Default for Type {
    fn default() -> Type {
        Self::File
    }
}
