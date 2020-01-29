use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InventoryConfig {
    destination: Destination,
    is_enabled: bool,
    filter: InventoryFilter,
    id: String,
    version: String,
    fields: OptionalFields,
    schedule: InventorySchedule,
}

#[derive(Debug, Deserialize)]
pub struct Destination {
    #[serde(rename = "S3BucketDestination")]
    destination: BucketDestination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketDestination {
    account_id: Option<String>,
    bucket: String,
    encryption: Option<InventoryEncryption>,
    format: InventoryFormat,
    prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum InventoryFormat {
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "Parquet")]
    Parquet,
}

#[derive(Debug, Deserialize)]
pub struct OptionalFields {
    #[serde(rename = "Field")]
    fields: Vec<InventoryFields>,
}

#[derive(Debug, Deserialize)]
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
    frequency: InventoryFrequency,
}

#[derive(Debug, Deserialize)]
pub enum InventoryFrequency {
    Daily,
    Weekly,
}
