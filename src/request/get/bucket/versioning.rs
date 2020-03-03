use crate::types::BucketVersioning;

impl_sub_resource!(GetBucketVersioning => BucketVersioning);

impl<'a> GetBucketVersioning<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketVersioning(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::VERSIONING, None)],
        })
    }
}
