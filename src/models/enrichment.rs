use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Enrichment {
    CreditDerivative(Box<models::EnrichmentFieldsCreditDerivative>),
    EntityOrg(Box<models::EnrichmentFieldsEntityOrg>),
    EntityPerson(Box<models::EnrichmentFieldsEntityPerson>),
    Equity(Box<models::EnrichmentFieldsEquity>),
    FixedIncomeCash(Box<models::EnrichmentFieldsFixedIncomeCash>),
    Fx(Box<models::EnrichmentFieldsFx>),
    FxCash(Box<models::EnrichmentFieldsFxCash>),
    FxDerivative(Box<models::EnrichmentFieldsFxDerivative>),
    Intent(Box<models::EnrichmentFieldsIntent>),
    Mortgage(Box<models::EnrichmentFieldsMortgage>),
    Option(Box<models::EnrichmentFieldsOption>),
    Other(Box<models::EnrichmentFieldsOther>),
    Repo(Box<models::EnrichmentFieldsRepo>),
    SecurityLending(Box<models::EnrichmentFieldsSecurityLending>),
}

impl Default for Enrichment {
    fn default() -> Self {
        Self::CreditDerivative(Default::default())
    }
}
