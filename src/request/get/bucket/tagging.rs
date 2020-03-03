use crate::types::BucketTagging;

impl_sub_resource!(GetBucketTagging => BucketTagging);

impl<'a> GetBucketTagging<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketTagging(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::TAGGING, None)],
        })
    }
}
