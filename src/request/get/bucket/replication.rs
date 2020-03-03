use crate::types::ReplicationConfiguration;

impl_sub_resource!(GetBucketReplicationConfig => ReplicationConfiguration);

impl<'a> GetBucketReplicationConfig<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketReplicationConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::REPLICATION, None)],
        })
    }
}
