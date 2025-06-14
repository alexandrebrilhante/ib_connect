use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostInitiateChat202Response {
    #[serde(rename = "recipients")]
    pub recipients: Vec<models::PostInitiateChat202ResponseRecipientsInner>,
    /// External Blast Id from chat initiation.
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl PostInitiateChat202Response {
    pub fn new(
        recipients: Vec<models::PostInitiateChat202ResponseRecipientsInner>,
    ) -> PostInitiateChat202Response {
        PostInitiateChat202Response {
            recipients,
            external_id: None,
        }
    }
}
