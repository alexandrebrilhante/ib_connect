use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextFormattingOptions {
    /// Text with strong importance, seriousness, or urgency.
    #[serde(rename = "strong", skip_serializing_if = "Option::is_none")]
    pub strong: Option<bool>,
    /// Text that has a stressed emphasis compared to surrounding text.
    #[serde(rename = "em", skip_serializing_if = "Option::is_none")]
    pub em: Option<bool>,
    /// Text which is marked or highlighted for reference or notation purposes.
    #[serde(rename = "mark", skip_serializing_if = "Option::is_none")]
    pub mark: Option<bool>,
}

impl TextFormattingOptions {
    pub fn new() -> TextFormattingOptions {
        TextFormattingOptions {
            strong: None,
            em: None,
            mark: None,
        }
    }
}
