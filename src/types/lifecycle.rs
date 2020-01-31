use crate::StorageClass;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketLifecycleOutput")]
pub struct BucketLifecycle {
    #[serde(rename = "Rule")]
    pub rules: Vec<LifecycleRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketLifecycleConfigurationOutput")]
pub struct BucketLifecycleConfig {
    #[serde(rename = "Rule")]
    pub rules: Vec<LifecycleRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecyleExpiration>,

    #[serde(rename = "ID")]
    pub id: Option<String>,

    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transition: Option<NoncurrentVersionExpiration>,
    pub prefix: String,
    pub status: LifecycleStatus,

    pub transition: Option<Transition>,
}

#[derive(Debug, Deserialize)]
pub enum LifecycleStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    pub days: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecyleExpiration {
    pub date: Option<String>,
    pub days: Option<u32>,
    pub expired_object_delete_marker: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionExpiration {
    pub noncurrent_days: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionTransition {
    pub noncurrent_days: Option<u32>,
    pub storage_class: Option<StorageClass>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transition {
    date: Option<String>,
    days: Option<u32>,
    storage_class: Option<StorageClass>,
}
