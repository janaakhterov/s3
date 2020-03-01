use crate::{
    request::list_buckets::Owner,
    types::Grant,
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

impl_sub_resource!(GetBucketAcl => BucketAcl);

impl<'a> GetBucketAcl<'a> {
    /// Create a new GetBucketAcl request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketAcl(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::ACL, None)],
        })
    }
}
