use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFieldsOptionInstrumentLegsInnerLinkedSecurity {
    /// Primary security FIGI identifier
    #[serde(rename = "figi", skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_description: Option<String>,
}

impl EnrichmentFieldsOptionInstrumentLegsInnerLinkedSecurity {
    pub fn new() -> EnrichmentFieldsOptionInstrumentLegsInnerLinkedSecurity {
        EnrichmentFieldsOptionInstrumentLegsInnerLinkedSecurity {
            figi: None,
            security_description: None,
        }
    }
}
