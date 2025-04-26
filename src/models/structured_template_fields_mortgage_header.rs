use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFieldsMortgageHeader {
    /// Application meta-data
    #[serde(rename = "app_meta_data", skip_serializing_if = "Option::is_none")]
    pub app_meta_data: Option<String>,
    /// The application originally generated the SPW pill
    #[serde(rename = "origin_app", skip_serializing_if = "Option::is_none")]
    pub origin_app: Option<OriginApp>,
    /// Version of the enrichment schema <major>.<minor>[.<patch>]
    #[serde(rename = "version")]
    pub version: String,
}

impl StructuredTemplateFieldsMortgageHeader {
    pub fn new(version: String) -> StructuredTemplateFieldsMortgageHeader {
        StructuredTemplateFieldsMortgageHeader {
            app_meta_data: None,
            origin_app: None,
            version,
        }
    }
}
/// The application originally generated the SPW pill
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OriginApp {
    #[serde(rename = "IB")]
    Ib,
    #[serde(rename = "TSOX")]
    Tsox,
}

impl Default for OriginApp {
    fn default() -> OriginApp {
        Self::Ib
    }
}
