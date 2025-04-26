use crate::models;
use serde::{Deserialize, Serialize};

/// Link : A link with optional display text.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "destination")]
    pub destination: Box<models::MarketCommentaryBodyLink>,
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<models::LinkTokensInner>>,
}

impl Link {
    /// A link with optional display text.
    pub fn new(r#type: Type, destination: models::MarketCommentaryBodyLink) -> Link {
        Link {
            r#type,
            destination: Box::new(destination),
            tokens: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Link")]
    Link,
}

impl Default for Type {
    fn default() -> Type {
        Self::Link
    }
}
