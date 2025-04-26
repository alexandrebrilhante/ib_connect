use crate::models;
use serde::{Deserialize, Serialize};

/// Paragraph : Represents a distinct section of text.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paragraph {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "tokens")]
    pub tokens: Vec<models::    use serde::{Deserialize, Serialize};
}

impl Paragraph {
    /// Represents a distinct section of text.
    pub fn new(r#type: Type, tokens: Vec<models::InlineToken>) -> Paragraph {
        Paragraph { r#type, tokens }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Paragraph")]
    Paragraph,
}

impl Default for Type {
    fn default() -> Type {
        Self::Paragraph
    }
}
