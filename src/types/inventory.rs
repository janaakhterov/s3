use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryConfig {
    inventory_destination: InventoryDestination,
    inventory_filter: InventoryFilter,

    #[serde(rename = "ID")]
    id: String,

    included_object_versions: String,
    is_enabled: bool,
    optional_fields: OptionalFields,
    schedule: InventorySchedule,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryDestination {
    #[serde(rename = "S3BucketDestination")]
    s3_bucket_destination: InventoryS3BucketDestination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryS3BucketDestination {
    account_id: Option<String>,
    bucket: String,
    inventory_encryption: Option<InventoryEncryption>,
    format: String,
    prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OptionalFields {
    #[serde(rename = "Field")]
    fields: Vec<InventoryFields>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InventoryFields {
    Size,
    LastModifiedDate,
    StorageClass,
    ETag,
    IsMultipartUploaded,
    ReplicationStatus,
    EncryptionStatus,
    ObjectLockRetainUntilDate,
    ObjectLockMode,
    ObjectLockLegalHoldStatus,
    IntelligentTieringAccessTier,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryEncryption {
    #[serde(rename = "SSEKMS")]
    sse_kms: Option<SseKms>,

    #[serde(rename = "SSES3")]
    sse_s3: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SseKms {
    key_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryFilter {
    prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventorySchedule {
    frequency: String,
}
