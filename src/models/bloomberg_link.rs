use crate::models;
use serde::{Deserialize, Serialize};

/// BloombergLink : Represents a command which runs a Bloomberg function.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BloombergLink {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "mnemonic")]
    pub mnemonic: String,
    #[serde(rename = "tails", skip_serializing_if = "Option::is_none")]
    pub tails: Option<Vec<String>>,
    #[serde(rename = "securities", skip_serializing_if = "Option::is_none")]
    pub securities: Option<Vec<String>>,
}

impl BloombergLink {
    /// Represents a command which runs a Bloomberg function.
    pub fn new(r#type: Type, mnemonic: String) -> BloombergLink {
        BloombergLink {
            r#type,
            mnemonic,
            tails: None,
            securities: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BBLink")]
    BbLink,
}

impl Default for Type {
    fn default() -> Type {
        Self::BbLink
    }
}
