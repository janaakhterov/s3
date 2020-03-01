use crate::types::BucketLocation;

impl_sub_resource!(GetBucketLocation => BucketLocation);

impl<'a> GetBucketLocation<'a> {
    /// Create a new GetBucketLocation request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketLocation(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::LOCATION, None)],
        })
    }
}
