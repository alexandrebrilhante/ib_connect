use crate::models;
use serde::{Deserialize, Serialize};

/// BlastInfo : Information about the blast event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlastInfo {
    /// External Blast Id from chat initiation.
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl BlastInfo {
    /// Information about the blast event
    pub fn new() -> BlastInfo {
        BlastInfo { external_id: None }
    }
}
