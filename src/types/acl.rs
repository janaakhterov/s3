use super::{
    Grant,
    Owner,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BucketAcl {
    #[serde(rename = "Owner")]
    owner: Owner,

    #[serde(rename = "AccessControlList")]
    list: AccessControlList,
}

#[derive(Debug, Deserialize)]
pub struct AccessControlList {
    #[serde(rename = "Grant")]
    grants: Vec<Grant>,
}
