use crate::types::BucketEncryption;

impl_sub_resource!(GetBucketEncryption => BucketEncryption);

impl<'a> GetBucketEncryption<'a> {
    /// Create a new GetBucketEncryption request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketEncryption(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::ENCRYPTION, None)],
        })
    }
}
