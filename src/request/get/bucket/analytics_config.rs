use crate::types::BucketAnalytics;

impl_sub_resource!(GetBucketAnalyticsConfig => BucketAnalytics);

impl<'a> GetBucketAnalyticsConfig<'a> {
    /// Create a new GetBucketAnalyticsConfig request with default parameters
    pub fn new(bucket: &'a str, id: &'a str) -> Self {
        GetBucketAnalyticsConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![
                (QueryParameter::ANALYTICS, None),
                (QueryParameter::ID, Some(id)),
            ],
        })
    }
}
