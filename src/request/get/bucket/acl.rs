use crate::types::BucketAcl;

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
