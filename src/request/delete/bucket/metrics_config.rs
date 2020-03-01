impl_sub_resource!(DeleteBucketMetricsConfig => ());

impl<'a> DeleteBucketMetricsConfig<'a> {
    /// Create a new DeleteBucketMetricsConfig request with default parameters
    pub fn new(bucket: &'a str, metrics_id: &'a str) -> Self {
        DeleteBucketMetricsConfig(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![
                (QueryParameter::METRICS, None),
                (QueryParameter::ID, Some(metrics_id)),
            ],
        })
    }
}
