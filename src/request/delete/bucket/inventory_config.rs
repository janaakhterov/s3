impl_sub_resource!(DeleteBucketInventoryConfig => ());

impl<'a> DeleteBucketInventoryConfig<'a> {
    /// Create a new DeleteBucketInventoryConfig request with default parameters
    pub fn new(bucket: &'a str, inventory_id: &'a str) -> Self {
        DeleteBucketInventoryConfig(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![
                (QueryParameter::INVENTORY, None),
                (QueryParameter::ID, Some(inventory_id)),
            ],
        })
    }
}
