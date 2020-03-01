impl_sub_resource!(DeleteBucketWebsite => ());

impl<'a> DeleteBucketWebsite<'a> {
    /// Create a new DeleteBucketWebsite request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketWebsite(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::WEBSITE, None)],
        })
    }
}
