use crate::models;
use serde::{Deserialize, Serialize};

/// TextCellContent : An element that represents text in an Table Cell.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextCellContent {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<models::TextCellContentStyle>>,
}

impl TextCellContent {
    /// An element that represents text in an Table Cell.
    pub fn new(text: String) -> TextCellContent {
        TextCellContent { text, style: None }
    }
}
