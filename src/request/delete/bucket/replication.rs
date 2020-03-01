impl_sub_resource!(DeleteBucketReplication => ());

impl<'a> DeleteBucketReplication<'a> {
    /// Create a new DeleteBucketReplication request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketReplication(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::REPLICATION, None)],
        })
    }
}
