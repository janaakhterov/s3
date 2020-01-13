/// Amazon S3 supports a set of predefined grants, known as canned ACLs.
/// Each canned ACL has a predefined set of grantees and permissions.
/// The following table lists the set of canned ACLs and the associated
/// predefined grants.
pub enum Acl {
    /// Owner gets FULL_CONTROL. No one else has access rights (default).
    Private,

    /// Owner gets FULL_CONTROL. The AllUsers group gets READ access.
    PublicRead,

    /// Owner gets FULL_CONTROL. The AllUsers group gets READ and WRITE access.
    /// Granting this on a bucket is generally not recommended.
    PublicReadWrite,

    /// Owner gets FULL_CONTROL. Amazon EC2 gets READ access to GET an
    /// Amazon Machine Image (AMI) bundle from Amazon S3.
    AuthenticationRead,

    /// Owner gets FULL_CONTROL. The AuthenticatedUsers group gets READ access.
    AwsExecRead,

    /// Object owner gets FULL_CONTROL. Bucket owner gets READ access.
    /// If you specify this canned ACL when creating a bucket, Amazon S3 ignores it.
    BucketOwnerRead,

    /// Both the object owner and the bucket owner get FULL_CONTROL over the object.
    /// If you specify this canned ACL when creating a bucket, Amazon S3 ignores it.
    BucketOwnerFullControl,
}

impl Into<&'static str> for Acl {
    fn into(self) -> &'static str {
        match self {
            Acl::Private => "private",
            Acl::PublicRead => "public-read",
            Acl::PublicReadWrite => "public-read-write",
            Acl::AuthenticationRead => "authentication-read",
            Acl::AwsExecRead => "aws-exec-read",
            Acl::BucketOwnerRead => "bucket-owner-read",
            Acl::BucketOwnerFullControl => "bucket-owner-full-control",
        }
    }
}
