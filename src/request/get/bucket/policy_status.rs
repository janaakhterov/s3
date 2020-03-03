use crate::types::PolicyStatus;

impl_sub_resource!(GetBucketPolicyStatus => PolicyStatus);

impl<'a> GetBucketPolicyStatus<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketPolicyStatus(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::POLICY_STATUS, None)],
        })
    }
}
