pub struct Headers;

#[allow(dead_code)]
impl Headers {
    pub const AUTHORIZATION: &'static str = "authorization";
    pub const CACHE_CONTROL: &'static str = "cache-control";
    pub const CONTENT_MD5: &'static str = "content-md5";
    pub const DELETE_MARKER: &'static str = "x-amz-delete-marker";
    pub const ETAG: &'static str = "etag";
    pub const EXPIRES: &'static str = "expires";
    pub const HOST: &'static str = "host";
    pub const IF_MATCH: &'static str = "if-match";
    pub const IF_MODIFIED_SINCE: &'static str = "if-modified-since";
    pub const IF_NONE_MATCH: &'static str = "if-none-match";
    pub const IF_UNMODIFIED_SINCE: &'static str = "if-unmodified-since";
    pub const LAST_MODIFIED: &'static str = "last-modified";
    pub const PARTS_COUNT: &'static str = "x-amz-mp-parts-count";
    pub const PART_NUMBER: &'static str = "part-number";
    pub const RANGE: &'static str = "range";
    pub const REQUEST_PAYER: &'static str = "request-payer";
    pub const RESPONSE_CACHE_CONTROL: &'static str = "response-cache-control";
    pub const RESPONSE_CONTENT_DISPOSITION: &'static str = "response-content-disposition";
    pub const RESPONSE_CONTENT_ENCODING: &'static str = "response-content-encoding";
    pub const RESPONSE_CONTENT_LANGUAGE: &'static str = "responsee-content-language";
    pub const RESPONSE_CONTENT_TYPE: &'static str = "response-content-type";
    pub const RESPONSE_EXPIRES: &'static str = "response-expires";
    pub const SSE_CUSTOMER_ALGORITHM: &'static str = "sse-customer-algorithm";
    pub const SSE_CUSTOMER_KEY: &'static str = "sse-customer-key";
    pub const SSE_CUSTOMER_KEY_MD5: &'static str = "sse-customer-key-md5";
    pub const VERSION_ID: &'static str = "vesrion-id";
    pub const X_AMZ_ACL: &'static str = "x-amz-acl";
    pub const X_AMZ_CONTENT_SHA256: &'static str = "x-amz-content-sha256";
    pub const X_AMZ_DATE: &'static str = "x-amz-date";
    pub const X_AMZ_STORAGE_CLASS: &'static str = "x-amz-storage-class";
    pub const X_AMZ_VERSION_ID: &'static str = "x-amz-version-id";
    pub const X_AMZ_GRANT_WRITE: &'static str = "x-amz-grant-write";
    pub const X_AMZ_GRANT_READ: &'static str = "x-amz-grant-read";
    pub const X_AMZ_GRANT_WRITE_ACP: &'static str = "x-amz-grant-write-acp";
    pub const X_AMZ_GRANT_READ_ACP: &'static str = "x-amz-grant-read-acp";
    pub const X_AMZ_GRANT_FULL_CONTROL: &'static str = "x-amz-grant-full-control";
}
