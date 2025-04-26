use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkTokensInner {}

impl LinkTokensInner {
    pub fn new() -> LinkTokensInner {
        LinkTokensInner {}
    }
}
