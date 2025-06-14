use crate::models;
use serde::{Deserialize, Serialize};

/// AudioAttachmentElement : Audio Attachment Element
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioAttachmentElement {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "audioEventId")]
    pub audio_event_id: String,
    #[serde(rename = "durationSecs")]
    pub duration_secs: f64,
}

impl AudioAttachmentElement {
    /// Audio Attachment Element
    pub fn new(r#type: Type, audio_event_id: String, duration_secs: f64) -> AudioAttachmentElement {
        AudioAttachmentElement {
            r#type,
            audio_event_id,
            duration_secs,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AUDIO")]
    Audio,
}

impl Default for Type {
    fn default() -> Type {
        Self::Audio
    }
}
