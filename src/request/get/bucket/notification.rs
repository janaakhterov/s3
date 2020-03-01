use crate::types::NotificationConfiguration;

impl_sub_resource!(GetBucketNotificationConfig => NotificationConfiguration);

impl<'a> GetBucketNotificationConfig<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketNotificationConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::NOTIFICATION, None)],
        })
    }
}
