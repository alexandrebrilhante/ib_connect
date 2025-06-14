use crate::models;
use serde::{Deserialize, Serialize};

/// IdeaHeaderProperties : Properties used to make the idea header customizable per client's preference.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdeaHeaderProperties {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    /// Client can set a short customized title for their idea which will be displayed in the idea header.
    #[serde(rename = "title")]
    pub title: String,
    /// Client can provide more context about their idea which will be displayed as a tooltip for the idea title.
    #[serde(rename = "description")]
    pub description: String,
}

impl IdeaHeaderProperties {
    /// Properties used to make the idea header customizable per client's preference.
    pub fn new(title: String, description: String) -> IdeaHeaderProperties {
        IdeaHeaderProperties {
            at_type: None,
            title,
            description,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "IdeaHeaderProperties")]
    IdeaHeaderProperties,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::IdeaHeaderProperties
    }
}
