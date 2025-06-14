use crate::models;
use serde::{Deserialize, Serialize};

/// TableCell : Cell element to construct Rows.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCell {
    #[serde(rename = "cell")]
    pub cell: Vec<models::CellContent>,
    /// The number of columns the cell should span.
    #[serde(rename = "colspan", skip_serializing_if = "Option::is_none")]
    pub colspan: Option<i32>,
    /// The number of rows the cell should span.
    #[serde(rename = "rowspan", skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<i32>,
    /// Horizonally align the content within the cell.
    #[serde(rename = "halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<Halign>,
}

impl TableCell {
    /// Cell element to construct Rows.
    pub fn new(cell: Vec<models::CellContent>) -> TableCell {
        TableCell {
            cell,
            colspan: None,
            rowspan: None,
            halign: None,
        }
    }
}
/// Horizonally align the content within the cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Halign {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Center")]
    Center,
    #[serde(rename = "Right")]
    Right,
}

impl Default for Halign {
    fn default() -> Halign {
        Self::Left
    }
}
