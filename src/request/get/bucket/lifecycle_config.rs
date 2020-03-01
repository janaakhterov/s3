use crate::types::BucketLifecycleConfig;

impl_sub_resource!(GetBucketLifecycleConfig => BucketLifecycleConfig);

impl<'a> GetBucketLifecycleConfig<'a> {
    /// Create a new GetBucketLifecycleConfig request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketLifecycleConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::LIFECYCLE, None)],
        })
    }
}
