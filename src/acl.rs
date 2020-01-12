pub enum Acl {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticationRead,
    AwsExecRead,
    BucketOwnerRead,
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
