use crate::types::BucketLogging;

impl_sub_resource!(GetBucketLogging => BucketLogging);

impl<'a> GetBucketLogging<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketLogging(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::LOGGING, None)],
        })
    }
}
