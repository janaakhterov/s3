use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "GetBucketVersioningOutput")]
#[serde(rename_all = "PascalCase")]
pub struct BucketVersioning {
    #[serde(rename = "MFADelete")]
    mfa_delete: String,
    status: String,
}
