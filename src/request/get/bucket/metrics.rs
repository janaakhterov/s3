use crate::types::BucketMetrics;

impl_sub_resource!(GetBucketMetrics => BucketMetrics);

impl<'a> GetBucketMetrics<'a> {
    /// Create a new GetBucketMetrics request with default parameters
    pub fn new(bucket: &'a str, id: &'a str) -> Self {
        GetBucketMetrics(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![
                (QueryParameter::METRICS, None),
                (QueryParameter::ID, Some(id)),
            ],
        })
    }
}
