use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsEntityPerson {
    #[serde(rename = "@type")]
    pub at_type: AtType,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<models::EnrichmentFieldsEntityPersonInstrumentInstrument>>,
}

impl EnrichmentFieldsEntityPerson {
    pub fn new(at_type: AtType) -> EnrichmentFieldsEntityPerson {
        EnrichmentFieldsEntityPerson {
            at_type,
            instrument: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "entity_person")]
    EntityPerson,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::EntityPerson
    }
}
