use crate::models;
use serde::{Deserialize, Serialize};

/// TableRow : Row element to construct Tables.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    #[serde(rename = "cells")]
    pub cells: Vec<models::TableCell>,
    /// Style the row as a table header.
    #[serde(rename = "isHeaderRow", skip_serializing_if = "Option::is_none")]
    pub is_header_row: Option<bool>,
}

impl TableRow {
    /// Row element to construct Tables.
    pub fn new(cells: Vec<models::TableCell>) -> TableRow {
        TableRow {
            cells,
            is_header_row: None,
        }
    }
}
