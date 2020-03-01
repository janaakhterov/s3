impl_sub_resource!(DeleteBucketTagging => ());

impl<'a> DeleteBucketTagging<'a> {
    /// Create a new DeleteBucketTagging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketTagging(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::TAGGING, None)],
        })
    }
}
