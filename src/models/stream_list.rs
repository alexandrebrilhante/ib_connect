use crate::models;
use serde::{Deserialize, Serialize};

/// StreamList : A list of available streams based on the credentials.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamList {
    #[serde(rename = "streams")]
    pub streams: Vec<models::StreamDesc>,
}

impl StreamList {
    /// A list of available streams based on the credentials.
    pub fn new(streams: Vec<models::StreamDesc>) -> StreamList {
        StreamList { streams }
    }
}
