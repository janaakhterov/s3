use super::Grantee;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "GetBucketLoggingOutput")]
pub struct BucketLogging {
    logging_enabled: LoggingEnabled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoggingEnabled {
    target_bucket: String,

    #[serde(rename = "TargetGrant")]
    target_grants: Vec<TargetGrant>,

    target_prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetGrant {
    grantee: Option<Grantee>,
    permission: Option<String>,
}
