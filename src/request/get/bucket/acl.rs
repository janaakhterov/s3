use crate::types::BucketAcl;

impl_sub_resource!(GetBucketAcl => BucketAcl, (ACL => None));

// impl<'a> GetBucketAcl<'a> {
//     /// Create a new GetBucketAcl request with default parameters
//     pub fn new(bucket: &'a str) -> Self {
//         Self { bucket }
//     }
// }
