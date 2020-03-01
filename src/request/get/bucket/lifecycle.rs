use crate::types::BucketLifecycle;

impl_sub_resource!(GetBucketLifecycle => BucketLifecycle);

impl<'a> GetBucketLifecycle<'a> {
    /// Create a new GetBucketLifecycle request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketLifecycle(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::LIFECYCLE, None)],
        })
    }
}
