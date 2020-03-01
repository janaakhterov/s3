impl_sub_resource!(DeleteBucketEncryption => ());

impl<'a> DeleteBucketEncryption<'a> {
    /// Create a new DeleteBucketEncryption request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketEncryption(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::ENCRYPTION, None)],
        })
    }
}
