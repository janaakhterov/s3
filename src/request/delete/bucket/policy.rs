impl_sub_resource!(DeleteBucketPolicy => ());

impl<'a> DeleteBucketPolicy<'a> {
    /// Create a new DeleteBucketPolicy request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketPolicy(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::POLICY, None)],
        })
    }
}
