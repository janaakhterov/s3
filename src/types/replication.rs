use super::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationConfiguration {
    role: String,
    rule: Vec<ReplicationRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationRule {
    delete_marker_replication: Option<DeleteMarkerReplication>,
    destination: ReplicationDestination,
    existing_object_replication: Option<ExistingObjectReplication>,
    filter: Option<ReplicationFilter>,
    id: Option<String>,
    prefix: Option<String>,
    priority: Option<i64>,
    source_selection_criteria: Option<SourceSelectionCriteria>,
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationDestination {
    access_control_translation: Option<AccessControlTranslation>,
    account: Option<String>,
    bucket: String,
    encryption_configuration: Option<EncryptionConfiguration>,
    metrics: Option<ReplicationMetrics>,
    replication_time: Option<ReplicationTime>,
    storage_class: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExistingObjectReplication {
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationFilter {
    and: Option<ReplicationRuleAndOperator>,
    prefix: Option<String>,
    tag: Option<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationRuleAndOperator {
    prefix: Option<String>,

    #[serde(rename = "Tag")]
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlTranslation {
    owner: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    replica_kms_key_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationMetrics {
    event_threshold: ReplicationTimeValue,
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationTimeValue {
    minutes: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationTime {
    status: String,
    time: ReplicationTimeValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMarkerReplication {
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SourceSelectionCriteria {
    sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SseKmsEncryptedObjects {
    status: String,
}
