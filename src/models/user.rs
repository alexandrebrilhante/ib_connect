use crate::models;
use serde::{Deserialize, Serialize};

/// User : An object that identifies a user by UUID. The field GPI has been DEPRECATED and will no longer be included in responses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "uuid")]
    pub uuid: i64,
    /// DEPRECATED and will no longer be included in responses.
    #[serde(
        rename = "gpi",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub gpi: Option<Option<String>>,
    #[serde(rename = "firmId", skip_serializing_if = "Option::is_none")]
    pub firm_id: Option<i64>,
    #[serde(
        rename = "firmName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub firm_name: Option<Option<String>>,
    #[serde(
        rename = "fullName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_name: Option<Option<String>>,
    #[serde(
        rename = "firstName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_name: Option<Option<String>>,
    #[serde(
        rename = "lastName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_name: Option<Option<String>>,
    #[serde(
        rename = "login",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub login: Option<Option<String>>,
}

impl User {
    /// An object that identifies a user by UUID. The field GPI has been DEPRECATED and will no longer be included in responses.
    pub fn new(uuid: i64) -> User {
        User {
            uuid,
            gpi: None,
            firm_id: None,
            firm_name: None,
            full_name: None,
            first_name: None,
            last_name: None,
            login: None,
        }
    }
}
