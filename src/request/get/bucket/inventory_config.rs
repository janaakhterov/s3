use crate::types::InventoryConfig;

impl_sub_resource!(GetBucketInventoryConfig => InventoryConfig);

impl<'a> GetBucketInventoryConfig<'a> {
    /// Create a new GetBucketInventoryConfig request with default parameters
    pub fn new(bucket: &'a str, id: &'a str) -> Self {
        GetBucketInventoryConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![
                (QueryParameter::INVENTORY, None),
                (QueryParameter::ID, Some(id)),
            ],
        })
    }
}
