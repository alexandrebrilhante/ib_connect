use crate::models;
use serde::{Deserialize, Serialize};

/// ComponentAttachmentElement : Launchpad Component Attachment Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentAttachmentElement {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "componentId")]
    pub component_id: String,
}

impl ComponentAttachmentElement {
    /// Launchpad Component Attachment Element
    pub fn new(r#type: Type, name: String, component_id: String) -> ComponentAttachmentElement {
        ComponentAttachmentElement {
            r#type,
            name,
            component_id,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "COMPONENT")]
    Component,
}

impl Default for Type {
    fn default() -> Type {
        Self::Component
    }
}
