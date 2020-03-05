use crate::Permission;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Grant {
    #[serde(rename = "Grantee")]
    grantee: Grantee,

    #[serde(rename = "Permission")]
    permission: Permission,
}

#[derive(Debug, Deserialize)]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    display_name: Option<String>,

    #[serde(rename = "EmailAddress")]
    email_address: Option<String>,

    #[serde(rename = "ID")]
    id: Option<String>,

    #[serde(rename = "xsi:type")]
    r#type: String,

    #[serde(rename = "URI")]
    uri: Option<String>,
}
