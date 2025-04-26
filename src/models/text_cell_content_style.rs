use crate::models;
use serde::{Deserialize, Serialize};

/// TextCellContentStyle : The styles to apply to the cell text.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextCellContentStyle {
    #[serde(rename = "decorations", skip_serializing_if = "Option::is_none")]
    pub decorations: Option<Vec<Decorations>>,
}

impl TextCellContentStyle {
    /// The styles to apply to the cell text.
    pub fn new() -> TextCellContentStyle {
        TextCellContentStyle { decorations: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Decorations {
    #[serde(rename = "Bold")]
    Bold,
    #[serde(rename = "Italic")]
    Italic,
    #[serde(rename = "Underline")]
    Underline,
    #[serde(rename = "Strikethrough")]
    Strikethrough,
}

impl Default for Decorations {
    fn default() -> Decorations {
        Self::Bold
    }
}
