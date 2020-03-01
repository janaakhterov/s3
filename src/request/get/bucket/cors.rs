use crate::types::BucketCors;

impl_sub_resource!(GetBucketCors => BucketCors);

impl<'a> GetBucketCors<'a> {
    /// Create a new GetBucketCors request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketCors(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::CORS, None)],
        })
    }
}
