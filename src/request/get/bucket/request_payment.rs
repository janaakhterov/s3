use crate::types::RequestPayment;

impl_sub_resource!(GetBucketRequestPayment => RequestPayment);

impl<'a> GetBucketRequestPayment<'a> {
    /// Create a new GetBucketLogging request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketRequestPayment(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::REQUEST_PAYMENT, None)],
        })
    }
}
