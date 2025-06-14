use crate::models;
use serde::{Deserialize, Serialize};

/// Trigger : Represents what caused the event to happen. The Trigger enum indicates why the messages was sent to the stream. A value of AUTOMATIC indicates that the messages was sent through the Base Feed or Enriched Feed workflow. Values beginning with MANUAL_ correspond to user actions in the On Demand workflow.
/// Represents what caused the event to happen. The Trigger enum indicates why the messages was sent to the stream. A value of AUTOMATIC indicates that the messages was sent through the Base Feed or Enriched Feed workflow. Values beginning with MANUAL_ correspond to user actions in the On Demand workflow.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Trigger {
    #[serde(rename = "AUTOMATIC")]
    Automatic,
    #[serde(rename = "MANUAL_SEND_TO_INTERNAL")]
    ManualSendToInternal,
}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Automatic => write!(f, "AUTOMATIC"),
            Self::ManualSendToInternal => write!(f, "MANUAL_SEND_TO_INTERNAL"),
        }
    }
}

impl Default for Trigger {
    fn default() -> Trigger {
        Self::Automatic
    }
}
