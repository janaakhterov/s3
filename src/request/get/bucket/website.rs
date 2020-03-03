use crate::types::BucketWebsite;

impl_sub_resource!(GetBucketWebsite => BucketWebsite);

impl<'a> GetBucketWebsite<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketWebsite(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::WEBSITE, None)],
        })
    }
}
