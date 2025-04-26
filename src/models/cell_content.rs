use crate::models;
use serde::{Deserialize, Serialize};

/// CellContent : An element that represents content in an Table Cell.
/// An element that represents content in an Table Cell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CellContent {
    TextCellContent(Box<models::TextCellContent>),
}

impl Default for CellContent {
    fn default() -> Self {
        Self::TextCellContent(Default::default())
    }
}
