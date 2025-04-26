use crate::models;
use serde::{Deserialize, Serialize};

/// StructuredTemplate : An object that contains a list of fields in a template that can be sent as structured content into IB Chats, and can therefore be rendered in a pill format in IB.
/// An object that contains a list of fields in a template that can be sent as structured content into IB Chats, and can therefore be rendered in a pill format in IB.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StructuredTemplate {
    Equity(Box<models::StructuredTemplateFieldsEquity>),
    FixedIncomeCash(Box<models::StructuredTemplateFieldsFixedIncomeCash>),
    Mortgage(Box<models::StructuredTemplateFieldsMortgage>),
    Option(Box<models::StructuredTemplateFieldsOption>),
    Other(Box<models::StructuredTemplateFieldsOther>),
    SecurityLending(Box<models::StructuredTemplateFieldsSecurityLending>),
}

impl Default for StructuredTemplate {
    fn default() -> Self {
        Self::Equity(Default::default())
    }
}
