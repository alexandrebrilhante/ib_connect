use crate::models;
use serde::{Deserialize, Serialize};

/// FileAttachmentElement : File Attachment Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileAttachmentElement {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: i32,
}

impl FileAttachmentElement {
    /// File Attachment Element
    pub fn new(r#type: Type, name: String, size_bytes: i32) -> FileAttachmentElement {
        FileAttachmentElement {
            r#type,
            name,
            size_bytes,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FILE")]
    File,
}

impl Default for Type {
    fn default() -> Type {
        Self::File
    }
}
