use super::{
    CommonPrefix,
    Owner,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketVersioningOutput")]
#[serde(rename_all = "PascalCase")]
pub struct BucketVersioning {
    #[serde(rename = "MFADelete")]
    mfa_delete: String,
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListObjectVersionsOutput")]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersions {
    common_prefixes: Vec<CommonPrefix>,
    delete_marker: Vec<DeleteMarkerEntry>,
    delimiter: String,
    encoding_type: String,
    is_truncated: bool,
    key_marker: String,
    max_keys: i32,
    name: String,
    next_key_marker: String,
    next_version_id_marker: String,
    prefix: String,
    version: Vec<ObjectVersion>,
    version_id_marker: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMarkerEntry {
    is_latest: Option<bool>,
    key: Option<String>,
    last_modified: Option<String>,
    owner: Owner,
    version_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    etag: Option<String>,
    is_latest: Option<String>,
    key: Option<String>,
    last_modified: Option<String>,
    owner: Option<Owner>,
    size: Option<u32>,
    storage_class: Option<String>,
    version_id: Option<String>,
}
