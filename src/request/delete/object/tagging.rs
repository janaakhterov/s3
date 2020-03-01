impl_sub_resource!(DeleteObjectTagging => ());

impl<'a> DeleteObjectTagging<'a> {
    /// Create a new DeleteObjectTagging request with default parameters
    pub fn new(bucket: &'a str, key: &'a str) -> Self {
        DeleteObjectTagging(SubResource {
            bucket,
            method: Method::DELETE,
            key: Some(key),
            params: vec![(QueryParameter::TAGGING, None)],
        })
    }

    pub fn version_id(mut self, version_id: &'a str) -> Self {
        self.0
            .params
            .push((QueryParameter::VERSION_ID, Some(version_id)));
        self
    }
}
