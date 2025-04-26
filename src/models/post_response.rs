use crate::models;
use serde::{Deserialize, Serialize};

/// PostResponse : Used to post a response.  The @type parameter needs to be \"IDEA\" or \"UIDEA\". The eventCorrelationId must match an eventId from an earlier ContentEvent for \"IDEA\".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum PostResponse {
    #[serde(rename = "IDEA")]
    Idea(Box<models::Idea>),
    #[serde(rename = "UIDEA")]
    Uidea(Box<models::Uidea>),
}

impl Default for PostResponse {
    fn default() -> Self {
        Self::Idea(Default::default())
    }
}
