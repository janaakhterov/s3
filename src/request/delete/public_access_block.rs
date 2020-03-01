impl_sub_resource!(DeletePublicAccessBlock => ());

impl<'a> DeletePublicAccessBlock<'a> {
    /// Create a new DeletePublicAccessBlock request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeletePublicAccessBlock(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::PUBLIC_ACCESS_BLOCK, None)],
        })
    }
}
